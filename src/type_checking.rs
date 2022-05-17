struct TypeChecker {
    scope: ScopeNode,
    ast: LNode,
}

impl TypeChecker {
    fn new(scope: ScopeNode, ast: LNode) -> Self {
        Self {
            scope,
            ast,
        }
    }

    fn type_check(&self) {

    }

    fn analysis(&self, node: LNode) {
        
    }
}