pub struct Attrib {
    name: String,
    val: String,
}

impl Attrib {
    pub fn new(nm: &str) -> Attrib {
        Attrib {
            name: String::from(nm),
            val: String::new(),
        }
    }
    pub fn to_string(&self) -> String {
        format!("{}=\"{}\" ", self.name, self.val)
    }
}

pub struct Tag {
    name: String,
    value: String,
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
                "<{} value=\"{}\" {}>{}</{}>",
                self.name,
                self.value,
                self.attrib2string(),
                self.child2string(),
                self.name
            )
        } else {
            format!(
                "<{} {}>{}</{}>",
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
            .join(" ")
    }
    pub fn child2string(&self) -> String {
        self.children
            .iter()
            .map(|c| c.to_svg())
            .collect::<Vec<String>>()
            .join("")
    }
}
