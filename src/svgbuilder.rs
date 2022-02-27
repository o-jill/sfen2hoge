pub struct Attrib {
    name: String,
    val: String,
}

impl Attrib {
    pub fn new(nm: &str, val: String) -> Attrib {
        Attrib {
            name: String::from(nm),
            val: val,
        }
    }
    pub fn from(nm: &str, val: &str) -> Attrib {
        Attrib::new(nm, val.to_string())
    }
    pub fn to_string(&self) -> String {
        if self.val.is_empty() {
            format!(" {}", self.name)
        } else {
            format!(" {}=\"{}\"", self.name, self.val)
        }
    }
}

#[test]
fn attribtest() {
    let a = Attrib::new("test", String::from("val1"));
    assert_eq!(a.name, "test");
    assert_eq!(a.val, "val1");
    let a = Attrib::from("atrib", "atai");
    assert_eq!(a.name, "atrib");
    assert_eq!(a.val, "atai");
    assert_eq!(a.to_string(), " atrib=\"atai\"");
    let a = Attrib::from("checked", "");
    assert_eq!(a.name, "checked");
    assert_eq!(a.val, "");
    assert_eq!(a.to_string(), " checked");
    let a = Attrib::new("noval", String::new());
    assert_eq!(a.name, "noval");
    assert_eq!(a.val, "");
    assert_eq!(a.to_string(), " noval");
}

pub struct Tag {
    name: String,
    pub value: String,
    attribs: Vec<Attrib>,
    children: Vec<Tag>,
}

impl Tag {
    pub fn new(nm: &str) -> Tag {
        Tag {
            name: String::from(nm),
            value: String::new(),
            attribs: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn addchild(&mut self, node: Tag) {
        self.children.push(node);
    }
    pub fn addattrib(&mut self, atr: Attrib) {
        self.attribs.push(atr);
    }

    pub fn newattrib(&mut self, nm: &str, val: &str) {
        self.addattrib(Attrib::from(nm, val));
    }

    pub fn to_svg(&self, indent: &str) -> String {
        if self.children.len() > 0 {
            format!(
                "{ind}<{nm}{val}{atr}>\n{chld}{ind}</{nm}>\n",
                nm = self.name,
                val = if self.value.is_empty() {
                    String::new()
                } else {
                    format!(" value=\"{}\"", self.value)
                },
                atr = self.attrib2string(),
                chld = self.child2string(indent.clone()),
                ind = indent
            )
        } else if self.value.is_empty() {
            format!("{}<{}{}/>\n", indent, self.name, self.attrib2string(),)
        } else {
            format!(
                "{}<{}{}>{}</{}>\n",
                indent,
                self.name,
                self.attrib2string(),
                self.value,
                self.name
            )
        }
    }
    pub fn attrib2string(&self) -> String {
        self.attribs
            .iter()
            .map(|a| a.to_string())
            .collect::<Vec<String>>()
            .join("")
    }
    pub fn child2string(&self, indent: &str) -> String {
        self.children
            .iter()
            .map(|c| c.to_svg(&format!("{} ", indent)))
            .collect::<Vec<String>>()
            .join("")
    }
    pub fn has_child(&self) -> bool {
        !self.children.is_empty()
    }
}

#[test]
fn tagtest() {
    let mut t = Tag::new("tag");
    assert_eq!(t.name, "tag");
    assert_eq!(t.value, "");
    assert_eq!(t.attribs.len(), 0);
    assert_eq!(t.children.len(), 0);
    assert!(!t.has_child());
    assert_eq!(t.attrib2string(), "");
    assert_eq!(t.child2string("abc"), "");
    assert_eq!(t.to_svg("def"), "def<tag/>\n");

    t.value = String::from("vaaluue");
    assert_eq!(t.value, "vaaluue");
    assert_eq!(t.attribs.len(), 0);
    assert_eq!(t.children.len(), 0);
    assert!(!t.has_child());
    assert_eq!(t.attrib2string(), "");
    assert_eq!(t.child2string("bcd"), "");
    assert_eq!(t.to_svg("efg"), "efg<tag>vaaluue</tag>\n");

    t.newattrib("checkbox", "on");
    assert_eq!(t.attribs.len(), 1);
    assert_eq!(t.children.len(), 0);
    assert!(!t.has_child());
    assert_eq!(t.attrib2string(), " checkbox=\"on\"");
    assert_eq!(t.child2string("cde"), "");
    assert_eq!(t.to_svg("fgh"), "fgh<tag checkbox=\"on\">vaaluue</tag>\n");

    t.value = String::new();
    assert_eq!(t.attribs.len(), 1);
    assert_eq!(t.children.len(), 0);
    assert!(!t.has_child());
    assert_eq!(t.attrib2string(), " checkbox=\"on\"");
    assert_eq!(t.child2string("ghi"), "");
    assert_eq!(t.to_svg("jkl"), "jkl<tag checkbox=\"on\"/>\n");

    t.addchild(Tag::new("child"));
    assert_eq!(t.attribs.len(), 1);
    assert_eq!(t.children.len(), 1);
    assert!(t.has_child());
    assert_eq!(t.attrib2string(), " checkbox=\"on\"");
    assert_eq!(t.child2string("hij"), "hij <child/>\n");
    assert_eq!(
        t.to_svg("klm"),
        "klm<tag checkbox=\"on\">\nklm <child/>\nklm</tag>\n"
    );
}

pub struct SVG {
    pub tag: Tag,
}

impl SVG {
    pub fn new() -> SVG {
        let mut svg = SVG {
            tag: Tag::new("svg"),
        };
        let atb = [
            ("width", "260"),
            ("height", "275"),
            ("viewBox", "0 0 260 275"),
            ("version", "1.1"),
            ("xmlns", "http://www.w3.org/2000/svg"),
        ];
        for (nm, val) in atb {
            svg.tag.newattrib(nm, val);
        }
        svg
    }
    pub fn to_string(&self) -> String {
        format!("<?xml version='1.0'?>\n{}", self.tag.to_svg(""))
    }
}
