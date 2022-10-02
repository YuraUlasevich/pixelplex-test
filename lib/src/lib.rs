

#[derive(Debug)]
pub struct Node<T> {
    pub id: u32,
    pub value: T,
}

#[derive(Debug)]
pub struct Relation{
    begin: u32, //from node
    end: u32, //to node
}

#[derive(Debug)]
pub struct Graph<T> {
    nodes: Vec<Node<T>>,
    relations: Vec<Relation>
}

impl<T> Node<T>
where T: std::fmt::Display + std::str::FromStr
{
    pub fn new(id: u32, value: T) -> Self {
        Self{id, value}
    }
    
    pub fn marshal(&self) -> String {
        format!("{} {}", self.id, self.value)
    }

    pub fn unmarshal(s: &str) -> Self {
        let spl = s.split(' ').collect::<Vec<&str>>();
        if spl.len() != 2 {
            panic!("parsing error of \"{}\"", s);
        } else {
            let id = spl[0].parse().expect(&format!("wrong id in \"{}\"", s).to_owned());
            match spl[1].parse() {
                Ok(value) => Self {id, value},
                Err(_) => panic!("can't parse value in \"{}\"", s)
            }
        }
    }
}
impl Relation {
    pub fn new(begin: u32, end: u32) -> Self {
        Self{begin, end}
    }
    
    pub fn marshal(&self) -> String {
        format!("{} {}", self.begin, self.end)
    }

    pub fn unmarshal(s: &str) -> Self {
        let spl = s.split(' ').collect::<Vec::<&str>>();
        if spl.len() != 2 {
            panic!("parsing error of \"{}\"", s);
        }
        let err = format!("parsing error in \"{}\"", s);
        Self {begin: spl[0].parse().expect(&err), end: spl[1].parse().expect(&err)}
    }
}
impl<T> Graph<T>
where T: std::fmt::Display + std::str::FromStr
{
    pub fn new() -> Self {
        Self {nodes: vec![], relations: vec![]}
    }
    pub fn add_node_from(mut self, node: Node<T>) -> Self {
        if self.nodes.iter().any(|n| n.id == node.id) {
            panic!("duplicate node with id {}", node.id);
        }
        self.nodes.push(node);
        self
    }
    pub fn add_node(self, id: u32, value: T) -> Self {
        self.add_node_from(Node::new(id, value))
    }
    pub fn remove_node_by_id(mut self, node_id: u32) -> Self {
        self.relations.retain(|e| !(node_id == e.begin || node_id == e.end));
        self.nodes.retain(|n| n.id != node_id);
        self
    }
    pub fn add_relation_from(mut self, relations: Relation) -> Self {
        if let None = self.nodes.iter().find(|n| relations.begin == n.id).and(self.nodes.iter().find(|n| relations.end == n.id)) {
            panic!("adding relation between non-existent nodes ({}->{})", relations.begin, relations.end);
        }
        if !self.relations.iter().any(|e| e.begin == relations.begin && e.end == relations.end) {
            self.relations.push(relations);
        }
        self
    }
    pub fn add_relation(self, begin: u32, end: u32) -> Self {
        self.add_relation_from(Relation::new(begin, end))
    }
    pub fn remove_relation(mut self, relation: &Relation) -> Self {
        self.relations.retain(|r| r.begin != relation.begin && r.end == relation.end);
        self
    }
    pub fn marshal(&self) -> String {
        let mut res = String::new();
        for node in &self.nodes {
            res.push_str(&node.marshal());
            res.push('\n');
        }
        res.push_str("#\n");
        for relation in &self.relations {
            res.push_str(&relation.marshal());
            res.push('\n');
        }
        res
    }
    pub fn unmarshal(s: &str) -> Self {
        let mut res = Self::new();
        let mut b: bool = true; // now parsing nodes
        for line in s.lines() {
            if line == "#" {
                b = false;
                continue;
            }
            if b {
                res = res.add_node_from(Node::unmarshal(&line));
            } else {
                res = res.add_relation_from(Relation::unmarshal(&line));
            }
        }
        
        res
    }
    pub fn get_connected<'a>(&'a self, node: &'a Node<T>) -> Vec::<&'a Node<T>> {
        let mut res = vec![];
        for relation in &self.relations {
            if relation.begin == node.id {
                if let Some(n) = self.nodes.iter().find(|nn| nn.id == relation.end) {
                    res.push(n);
                }
            }
        }
        
        res
    }
    
    pub fn pass_from<F>(&self, root: &Node<T>, f: &mut F)
    where F: FnMut(&Node<T>)
    {
        self.pass_from_with_memory(root, f, &vec![]);
    }

    /// DFS
    fn pass_from_with_memory<'a, F>(&'a self, root: &'a Node<T>, f: &mut F, memory: &'a Vec<&'a Node<T>>)
    where F: FnMut(&Node<T>)
    {
        if let Some(_) = memory.iter().find(|n| n.id == root.id) {
            return
        }
        f(root);
        let mut memory = memory.clone();
        memory.push(root);
        let nexts = self.get_connected(root);
        for next in nexts {
            self.pass_from_with_memory(next, f, &memory);
        }
    }

    pub fn get_all_nodes(&self) -> Vec::<&Node<T>> {
        self.nodes.iter().collect()
    }
}