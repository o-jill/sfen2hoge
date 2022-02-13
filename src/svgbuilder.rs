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
    pub fn to_string(&self) -> String {
        if self.val.is_empty() {
            self.name.clone()
        } else {
            format!(" {}=\"{}\"", self.name, self.val)
        }
    }
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
    pub fn to_svg(&self) -> String {
        if self.children.len() > 0 {
            format!(
                "<{}{}{}>\n{}\n</{}>",
                self.name,
                if self.value.is_empty() {
                    String::new()
                } else {
                    format!(" value=\"{}\"", self.value)
                },
                self.attrib2string(),
                self.child2string(),
                self.name
            )
        } else {
            format!(
                "<{}{}>{}</{}>",
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
    pub fn child2string(&self) -> String {
        self.children
            .iter()
            .map(|c| c.to_svg())
            .collect::<Vec<String>>()
            .join("")
    }
}

pub struct SVG {
    pub tag: Tag,
}

impl SVG {
    pub fn new() -> SVG {
        let mut svg = SVG {
            tag: Tag::new("svg"),
        };
        svg.tag
            .addattrib(Attrib::new("version", String::from("1.1")));
        svg.tag.addattrib(Attrib::new(
            "xmlns",
            String::from("http://www.w3.org/2000/svg"),
        ));
        svg
    }
    pub fn to_svg(&self) -> String {
        format!("<?xml version='1.0'?>\n{}", self.tag.to_svg())
    }
}
