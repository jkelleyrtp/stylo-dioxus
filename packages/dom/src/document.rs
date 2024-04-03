use crate::Node;
use selectors::{matching::QuirksMode, Element};
use slab::Slab;
use std::collections::HashMap;
use style::servo_arc::Arc as ServoArc;
use style::{
    dom::{TDocument, TNode},
    media_queries::{Device, MediaList},
    selector_parser::SnapshotMap,
    shared_lock::{SharedRwLock, StylesheetGuards},
    stylesheets::{AllowImportRules, DocumentStyleSheet, Origin, Stylesheet, UrlExtraData},
    stylist::Stylist,
};
use taffy::AvailableSpace;
use url::Url;

pub struct Document {
    /// A bump-backed tree
    ///
    /// Both taffy and stylo traits are implemented for this.
    /// We pin the tree to a guarantee to the nodes it creates that the tree is stable in memory.
    ///
    /// There is no way to create the tree - publicly or privately - that would invalidate that invariant.
    pub(crate) nodes: Box<Slab<Node>>,

    pub(crate) guard: SharedRwLock,

    /// The styling engine of firefox
    pub(crate) stylist: Stylist,

    // caching for the stylist
    pub(crate) snapshots: SnapshotMap,

    pub(crate) nodes_to_id: HashMap<String, usize>,

    /// Base url for resolving linked resources (stylesheets, images, fonts, etc)
    pub(crate) base_url: Option<url::Url>,
}

impl Document {
    pub fn new(device: Device) -> Self {
        let quirks = QuirksMode::NoQuirks;
        let stylist = Stylist::new(device, quirks);
        let snapshots = SnapshotMap::new();
        let nodes = Box::new(Slab::new());
        let guard = SharedRwLock::new();
        let nodes_to_id = HashMap::new();

        // Make sure we turn on servo features
        style_config::set_bool("layout.flexbox.enabled", true);
        style_config::set_bool("layout.legacy_layout", true);
        style_config::set_bool("layout.columns.enabled", true);

        Self {
            guard,
            nodes,
            stylist,
            snapshots,
            nodes_to_id,
            base_url: None,
        }
    }

    /// Set base url for resolving linked resources (stylesheets, images, fonts, etc)
    pub fn set_base_url(&mut self, url: &str) {
        self.base_url = Url::parse(url).ok();
    }

    pub fn tree(&self) -> &Slab<Node> {
        &self.nodes
    }

    pub fn root_node(&self) -> &Node {
        &self.nodes[0]
    }

    pub fn root_element(&self) -> &Node {
        TDocument::as_node(&self.root_node())
            .first_element_child()
            .unwrap()
            .as_element()
            .unwrap()
    }

    pub fn resolve_url(&self, raw: &str) -> url::Url {
        match &self.base_url {
            Some(base_url) => base_url.join(raw).unwrap(),
            None => url::Url::parse(raw).unwrap(),
        }
    }

    pub fn flush_child_indexes(&mut self, target_id: usize, child_idx: usize, level: usize) {
        let node = &mut self.nodes[target_id];
        node.child_idx = child_idx;

        // println!("{} {} {:?} {:?}", "  ".repeat(level), target_id, node.parent, node.children);

        for (i, child_id) in node.children.clone().iter().enumerate() {
            self.flush_child_indexes(*child_id, i, level + 1)
        }
    }

    pub fn print_tree(&self) {
        self.root_node().print_tree(0);
    }

    // pub fn populate_from_rc_dom(&mut self, children: &[Handle], parent: Option<usize>) {
    //     for (child_idx, node) in children.into_iter().enumerate() {
    //         // Create this node, absorbing any script/style data.
    //         let id = self.add_node(node, child_idx, parent);

    //         // Add this node to its parent's list of children.
    //         if let Some(parent) = parent {
    //             self.nodes[parent].children.push(id);
    //         }

    //         // Now go insert its children. We want their IDs to come back here so we know how to walk them.
    //         self.populate_from_rc_dom(&node.children.borrow(), Some(id));
    //     }
    // }

    // pub fn on_add_node(&mut self, node_id: usize) {
    //     let slab_ptr = self.nodes.as_mut() as *mut Slab<Node>;
    //     let entry = self.nodes.vacant_entry();
    //     let id = entry.key();
    //     let stylo_element_data: AtomicRefCell<Option<ElementData>> = Default::default();
    //     let style = Style::DEFAULT;

    //     let val = Node {
    //         id,
    //         style,
    //         display_outer: DisplayOuter::Block,
    //         child_idx,
    //         children: vec![],
    //         raw_dom_data: node.data,
    //         hidden: false,
    //         parent,
    //         cache: Cache::new(),
    //         stylo_element_data,
    //         unrounded_layout: Layout::new(),
    //         final_layout: Layout::new(),
    //         tree: slab_ptr,
    //         guard: self.guard.clone(),
    //     };

    //     let entry = entry.insert(val);

    //     let node = self.nodes[node_id];

    //     match &node.raw_dom_data {
    //         NodeData::Element(data) => {
    //             let ElementNodeData {
    //                 name,
    //                 template_contents,
    //                 ..
    //             } = data;

    //             //
    //             match name.local.as_ref() {
    //                 // Attach the style to the document
    //                 "style" => {
    //                     let mut css = String::new();
    //                     for child in node.children.borrow().iter() {
    //                         match &child.data {
    //                             NodeData::Text { contents } => {
    //                                 css.push_str(&contents.borrow().to_string());
    //                             }
    //                             _ => {}
    //                         }
    //                     }
    //                     // unescape the css
    //                     let css = html_escape::decode_html_entities(&css);
    //                     self.add_stylesheet(&css);
    //                 }

    //                 // Resolve external stylesheet
    //                 "link" => {
    //                     if entry.attr(local_name!("rel")) == Some("stylesheet") {
    //                         let raw_url = entry
    //                             .attr(local_name!("href"))
    //                             .map(ToString::to_string)
    //                             .unwrap_or_default();
    //                         let url = self.resolve_url(&raw_url);

    //                         match crate::util::fetch_string(url.as_str()) {
    //                             Ok(css) => {
    //                                 let css = html_escape::decode_html_entities(&css);
    //                                 self.add_stylesheet(&css);
    //                             }
    //                             Err(_) => {
    //                                 eprintln!("Error fetching stylesheet {}", url);
    //                             }
    //                         }
    //                     }
    //                 }

    //                 // Create a shadow element and attach it to this node
    //                 "input" => {
    //                     // get the value and/or placeholder:
    //                     let mut value = None;
    //                     let mut placeholder = None;
    //                     for attr in attrs.borrow().iter() {
    //                         match attr.name.local.as_ref() {
    //                             "value" => {
    //                                 value = Some(attr.value.to_string());
    //                             }
    //                             "placeholder" => {
    //                                 placeholder = Some(attr.value.to_string());
    //                             }
    //                             _ => {}
    //                         }
    //                     }

    //                     if let Some(value) = value {
    //                         let mut tendril: Tendril<html5ever::tendril::fmt::UTF8> =
    //                             Tendril::new();

    //                         tendril.write_str(value.as_str()).unwrap();

    //                         let contents: RefCell<Tendril<html5ever::tendril::fmt::UTF8>> =
    //                             RefCell::new(tendril);

    //                         let handle = Handle::new(markup5ever_rcdom::Node {
    //                             parent: Cell::new(Some(Rc::downgrade(node))),
    //                             children: Default::default(),
    //                             data: NodeData::Text { contents },
    //                         });

    //                         // inserted as a child of the input
    //                         let shadow = self.add_node(&handle, 0, Some(id));

    //                         // attach it to its parent
    //                         self.nodes[id].children.push(shadow);
    //                     }
    //                 }

    //                 // todo: Load images
    //                 "img" => {
    //                     if let Some(raw_src) = entry.attr(local_name!("src")) {
    //                         if raw_src.len() > 0 {
    //                             // let raw_src = raw_src.to_string();
    //                             let src = self.resolve_url(&raw_src);
    //                             let image_result = crate::util::fetch_image(src.as_str());

    //                             match image_result {
    //                                 Ok(image) => {
    //                                     self.nodes[id].element_data_mut().unwrap().image =
    //                                         Some(Arc::new(image));
    //                                 }
    //                                 Err(_) => {
    //                                     eprintln!("Error fetching image {}", src);
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }

    //                 // Todo: Load scripts
    //                 "script" => {}

    //                 // Load template elements (unpaired usually)
    //                 "template" => {
    //                     if let Some(template_contents) = template_contents.borrow().as_ref() {
    //                         let id = self
    //                             .populate_from_rc_dom(&template_contents.children.borrow(), None);
    //                     }
    //                 }

    //                 _ => entry.flush_style_attribute(),
    //             }
    //         }
    //         // markup5ever_rcdom::NodeData::Document => todo!(),
    //         // markup5ever_rcdom::NodeData::Doctype { name, public_id, system_id } => todo!(),
    //         // markup5ever_rcdom::NodeData::Text { contents } => todo!(),
    //         // markup5ever_rcdom::NodeData::Comment { contents } => todo!(),
    //         // markup5ever_rcdom::NodeData::ProcessingInstruction { target, contents } => todo!(),
    //         _ => {}
    //     }

    //     id
    // }

    pub fn process_style_element(&mut self, target_id: usize) {
        let css = self.nodes[target_id].text_content();
        let css = html_escape::decode_html_entities(&css);
        self.add_stylesheet(&css);
    }

    pub fn add_stylesheet(&mut self, css: &str) {
        let data = Stylesheet::from_str(
            css,
            UrlExtraData::from(
                "data:text/css;charset=utf-8;base64,"
                    .parse::<Url>()
                    .unwrap(),
            ),
            Origin::UserAgent,
            ServoArc::new(self.guard.wrap(MediaList::empty())),
            self.guard.clone(),
            None,
            None,
            QuirksMode::NoQuirks,
            0,
            AllowImportRules::Yes,
        );

        self.stylist
            .append_stylesheet(DocumentStyleSheet(ServoArc::new(data)), &self.guard.read());

        self.stylist
            .force_stylesheet_origins_dirty(Origin::Author.into());
    }

    /// Restyle the tree and then relayout it
    pub fn resolve(&mut self) {
        // we need to resolve stylist first since it will need to drive our layout bits
        self.resolve_stylist();

        // Merge stylo into taffy
        self.flush_styles_to_layout(vec![self.root_element().id], None, taffy::Display::Block);

        // Next we resolve layout with the data resolved by stlist
        self.resolve_layout();
    }

    // Takes (x, y) co-ordinates (relative to the )
    pub fn hit(&self, x: f32, y: f32) -> Option<usize> {
        self.root_element().hit(x, y)
    }

    /// Update the device and reset the stylist to process the new size
    pub fn set_stylist_device(&mut self, device: Device) {
        let guard = &self.guard;
        let guards = StylesheetGuards {
            author: &guard.read(),
            ua_or_user: &guard.read(),
        };
        let origins = self.stylist.set_device(device, &guards);
        self.stylist.force_stylesheet_origins_dirty(origins);
    }
    pub fn stylist_device(&mut self) -> &Device {
        self.stylist.device()
    }

    /// Walk the nodes now that they're properly styled and transfer their styles to the taffy style system
    /// Ideally we could just break apart the styles into ECS bits, but alas
    ///
    /// Todo: update taffy to use an associated type instead of slab key
    /// Todo: update taffy to support traited styles so we don't even need to rely on taffy for storage
    pub fn resolve_layout(&mut self) {
        let size = self.stylist.device().au_viewport_size();

        let available_space = taffy::Size {
            // width: AvailableSpace::MaxContent,
            // height: AvailableSpace::Definite(10000000.0),
            // width: AvailableSpace::Definite(dbg!(1200.0)),
            // height: AvailableSpace::Definite(dbg!(2000.0)),
            // };
            width: AvailableSpace::Definite(size.width.to_f32_px()),
            height: AvailableSpace::Definite(size.height.to_f32_px()),
            // height: AvailableSpace::Definite(1000000.0),
        };

        let root_node_id = taffy::NodeId::from(self.root_element().id);

        taffy::compute_root_layout(self, root_node_id, available_space);
        taffy::round_layout(self, root_node_id);
    }

    pub fn set_document(&mut self, _content: String) {}

    pub fn add_element(&mut self) {}
}

impl AsRef<Document> for Document {
    fn as_ref(&self) -> &Document {
        self
    }
}

impl AsMut<Document> for Document {
    fn as_mut(&mut self) -> &mut Document {
        self
    }
}
