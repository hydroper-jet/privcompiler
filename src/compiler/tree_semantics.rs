use crate::ns::*;
use by_address::ByAddress;

/// Structure that assigns semantic symbols to syntactic nodes.
pub struct TreeSemantics {
    compilation_units: RefCell<HashMap<ByAddress<Rc<CompilationUnit>>, TreeSemantics1>>,
}

struct TreeSemantics1 {
    expressions: HashMap<AstAsKey<Rc<Expression>>, Option<Symbol>>,
    directives: HashMap<AstAsKey<Rc<Directive>>, Option<Symbol>>,
    simple_variable_definitions: HashMap<AstAsKey<Rc<SimpleVariableDefinition>>, Option<Symbol>>,
    blocks: HashMap<AstAsKey<Rc<Block>>, Option<Symbol>>,
    programs: HashMap<AstAsKey<Rc<Program>>, Option<Symbol>>,
    function_commons: HashMap<AstAsKey<Rc<FunctionCommon>>, Option<Symbol>>,
}

impl TreeSemantics1 {
    fn new() -> Self {
        Self {
            expressions: HashMap::new(),
            directives: HashMap::new(),
            simple_variable_definitions: HashMap::new(),
            blocks: HashMap::new(),
            programs: HashMap::new(),
            function_commons: HashMap::new(),
        }
    }
}

impl TreeSemantics {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            compilation_units: RefCell::new(HashMap::new()),
        })
    }
}

pub trait TreeSemanticsAccessor<T> {
    fn get(&self, node: &Rc<T>) -> Option<Symbol>;
    fn set(&self, node: &Rc<T>, symbol: Option<Symbol>);
    fn delete(&self, node: &Rc<T>) -> bool;

    fn has(&self, node: &Rc<T>) -> bool {
        self.get(node).is_some()
    }
}

impl TreeSemanticsAccessor<Expression> for TreeSemantics {
    fn get(&self, node: &Rc<Expression>) -> Option<Symbol> {
        let compilation_units = self.compilation_units.borrow();
        let m1 = compilation_units.get(&ByAddress(node.location().compilation_unit()));
        if let Some(m1) = m1 {
            m1.expressions.get(&AstAsKey(node.clone())).map(|v| v.clone().unwrap())
        } else {
            None
        }
    }

    fn set(&self, node: &Rc<Expression>, symbol: Option<Symbol>) {
        let compilation_unit = node.location().compilation_unit();
        let mut compilation_units = self.compilation_units.borrow_mut();
        let m1 = compilation_units.get_mut(&ByAddress(compilation_unit.clone()));
        if let Some(m1) = m1 {
            m1.expressions.insert(AstAsKey(node.clone()), symbol);
        } else {
            let mut m1 = TreeSemantics1::new();
            m1.expressions.insert(AstAsKey(node.clone()), symbol);
            compilation_units.insert(ByAddress(compilation_unit), m1);
        }
    }

    fn delete(&self, node: &Rc<Expression>) -> bool {
        let compilation_unit = node.location().compilation_unit();
        let mut compilation_units = self.compilation_units.borrow_mut();
        let m1 = compilation_units.get_mut(&ByAddress(compilation_unit));
        if let Some(m1) = m1 {
            m1.expressions.remove(&AstAsKey(node.clone())).is_some()
        } else {
            false
        }
    }
}

impl TreeSemanticsAccessor<Directive> for TreeSemantics {
    fn get(&self, node: &Rc<Directive>) -> Option<Symbol> {
        let compilation_units = self.compilation_units.borrow();
        let m1 = compilation_units.get(&ByAddress(node.location().compilation_unit()));
        if let Some(m1) = m1 {
            m1.directives.get(&AstAsKey(node.clone())).map(|v| v.clone().unwrap())
        } else {
            None
        }
    }

    fn set(&self, node: &Rc<Directive>, symbol: Option<Symbol>) {
        let compilation_unit = node.location().compilation_unit();
        let mut compilation_units = self.compilation_units.borrow_mut();
        let m1 = compilation_units.get_mut(&ByAddress(compilation_unit.clone()));
        if let Some(m1) = m1 {
            m1.directives.insert(AstAsKey(node.clone()), symbol);
        } else {
            let mut m1 = TreeSemantics1::new();
            m1.directives.insert(AstAsKey(node.clone()), symbol);
            compilation_units.insert(ByAddress(compilation_unit), m1);
        }
    }

    fn delete(&self, node: &Rc<Directive>) -> bool {
        let compilation_unit = node.location().compilation_unit();
        let mut compilation_units = self.compilation_units.borrow_mut();
        let m1 = compilation_units.get_mut(&ByAddress(compilation_unit));
        if let Some(m1) = m1 {
            m1.directives.remove(&AstAsKey(node.clone())).is_some()
        } else {
            false
        }
    }
}

impl TreeSemanticsAccessor<SimpleVariableDefinition> for TreeSemantics {
    fn get(&self, node: &Rc<SimpleVariableDefinition>) -> Option<Symbol> {
        let compilation_units = self.compilation_units.borrow();
        let m1 = compilation_units.get(&ByAddress(node.location.compilation_unit()));
        if let Some(m1) = m1 {
            m1.simple_variable_definitions.get(&AstAsKey(node.clone())).map(|v| v.clone().unwrap())
        } else {
            None
        }
    }

    fn set(&self, node: &Rc<SimpleVariableDefinition>, symbol: Option<Symbol>) {
        let compilation_unit = node.location.compilation_unit();
        let mut compilation_units = self.compilation_units.borrow_mut();
        let m1 = compilation_units.get_mut(&ByAddress(compilation_unit.clone()));
        if let Some(m1) = m1 {
            m1.simple_variable_definitions.insert(AstAsKey(node.clone()), symbol);
        } else {
            let mut m1 = TreeSemantics1::new();
            m1.simple_variable_definitions.insert(AstAsKey(node.clone()), symbol);
            compilation_units.insert(ByAddress(compilation_unit), m1);
        }
    }

    fn delete(&self, node: &Rc<SimpleVariableDefinition>) -> bool {
        let compilation_unit = node.location.compilation_unit();
        let mut compilation_units = self.compilation_units.borrow_mut();
        let m1 = compilation_units.get_mut(&ByAddress(compilation_unit));
        if let Some(m1) = m1 {
            m1.simple_variable_definitions.remove(&AstAsKey(node.clone())).is_some()
        } else {
            false
        }
    }
}

impl TreeSemanticsAccessor<Block> for TreeSemantics {
    fn get(&self, node: &Rc<Block>) -> Option<Symbol> {
        let compilation_units = self.compilation_units.borrow();
        let m1 = compilation_units.get(&ByAddress(node.location.compilation_unit()));
        if let Some(m1) = m1 {
            m1.blocks.get(&AstAsKey(node.clone())).map(|v| v.clone().unwrap())
        } else {
            None
        }
    }

    fn set(&self, node: &Rc<Block>, symbol: Option<Symbol>) {
        let compilation_unit = node.location.compilation_unit();
        let mut compilation_units = self.compilation_units.borrow_mut();
        let m1 = compilation_units.get_mut(&ByAddress(compilation_unit.clone()));
        if let Some(m1) = m1 {
            m1.blocks.insert(AstAsKey(node.clone()), symbol);
        } else {
            let mut m1 = TreeSemantics1::new();
            m1.blocks.insert(AstAsKey(node.clone()), symbol);
            compilation_units.insert(ByAddress(compilation_unit), m1);
        }
    }

    fn delete(&self, node: &Rc<Block>) -> bool {
        let compilation_unit = node.location.compilation_unit();
        let mut compilation_units = self.compilation_units.borrow_mut();
        let m1 = compilation_units.get_mut(&ByAddress(compilation_unit));
        if let Some(m1) = m1 {
            m1.blocks.remove(&AstAsKey(node.clone())).is_some()
        } else {
            false
        }
    }
}

impl TreeSemanticsAccessor<Program> for TreeSemantics {
    fn get(&self, node: &Rc<Program>) -> Option<Symbol> {
        let compilation_units = self.compilation_units.borrow();
        let m1 = compilation_units.get(&ByAddress(node.location.compilation_unit()));
        if let Some(m1) = m1 {
            m1.programs.get(&AstAsKey(node.clone())).map(|v| v.clone().unwrap())
        } else {
            None
        }
    }

    fn set(&self, node: &Rc<Program>, symbol: Option<Symbol>) {
        let compilation_unit = node.location.compilation_unit();
        let mut compilation_units = self.compilation_units.borrow_mut();
        let m1 = compilation_units.get_mut(&ByAddress(compilation_unit.clone()));
        if let Some(m1) = m1 {
            m1.programs.insert(AstAsKey(node.clone()), symbol);
        } else {
            let mut m1 = TreeSemantics1::new();
            m1.programs.insert(AstAsKey(node.clone()), symbol);
            compilation_units.insert(ByAddress(compilation_unit), m1);
        }
    }

    fn delete(&self, node: &Rc<Program>) -> bool {
        let compilation_unit = node.location.compilation_unit();
        let mut compilation_units = self.compilation_units.borrow_mut();
        let m1 = compilation_units.get_mut(&ByAddress(compilation_unit));
        if let Some(m1) = m1 {
            m1.programs.remove(&AstAsKey(node.clone())).is_some()
        } else {
            false
        }
    }
}

impl TreeSemanticsAccessor<FunctionCommon> for TreeSemantics {
    fn get(&self, node: &Rc<FunctionCommon>) -> Option<Symbol> {
        let compilation_units = self.compilation_units.borrow();
        let m1 = compilation_units.get(&ByAddress(node.location.compilation_unit()));
        if let Some(m1) = m1 {
            m1.function_commons.get(&AstAsKey(node.clone())).map(|v| v.clone().unwrap())
        } else {
            None
        }
    }

    fn set(&self, node: &Rc<FunctionCommon>, symbol: Option<Symbol>) {
        let compilation_unit = node.location.compilation_unit();
        let mut compilation_units = self.compilation_units.borrow_mut();
        let m1 = compilation_units.get_mut(&ByAddress(compilation_unit.clone()));
        if let Some(m1) = m1 {
            m1.function_commons.insert(AstAsKey(node.clone()), symbol);
        } else {
            let mut m1 = TreeSemantics1::new();
            m1.function_commons.insert(AstAsKey(node.clone()), symbol);
            compilation_units.insert(ByAddress(compilation_unit), m1);
        }
    }

    fn delete(&self, node: &Rc<FunctionCommon>) -> bool {
        let compilation_unit = node.location.compilation_unit();
        let mut compilation_units = self.compilation_units.borrow_mut();
        let m1 = compilation_units.get_mut(&ByAddress(compilation_unit));
        if let Some(m1) = m1 {
            m1.function_commons.remove(&AstAsKey(node.clone())).is_some()
        } else {
            false
        }
    }
}