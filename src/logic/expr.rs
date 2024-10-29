#[macro_export]
macro_rules! propositional {
    ($expr: expr) => {
        Tree::<PLogic>::build($expr)
    };
}

#[macro_export]
macro_rules! first_order {
    ($expr: expr) => {
        Tree::<FirstOrderLogic>::build($expr)
    };
}

#[macro_export]
macro_rules! circuit {
    ($expr: expr) => {
        Tree::<ProbabilisticCircuit>::build($expr)
    };
}

#[macro_export]
macro_rules! var {
    ($id: tt) => {
        var!(id:$id)
    };
    (name:$name: tt) => {
        |builder| {
            let id = builder.add_named(&$name.to_string());
            builder.var(id)
        }
    };
    (id:$id: tt) => {
        |builder| builder.var($id)
    };
}

#[macro_export]
macro_rules! not {
    ($e: expr) => {
        |builder| builder.not($e)
    };
}

#[macro_export]
macro_rules! and {
    ($left: expr,$right: expr) => {
        |builder| builder.and($left, $right)
    };
}

#[macro_export]
macro_rules! or {
    ($left: expr,$right: expr) => {
        |builder| builder.or($left, $right)
    };
}

#[macro_export]
macro_rules! imply {
    ($left: expr,$right: expr) => {
        or!(not!($left), $right)
    };
}

#[macro_export]
macro_rules! equiv {
    ($left: expr,$right: expr) => {
        and!(or!(not!($left), $right), or!($left, not!($right)))
    };
}

#[macro_export]
macro_rules! conjunction {
    ($e: expr) => {
        $e
    };
    ($e:expr,$($es:expr),+) => {{
        and!($e, conjunction! ($($es),+))
    }};
}

#[macro_export]
macro_rules! disjunction {
    ($e: expr) => {
        $e
    };
    ($e:expr,$($es:expr),+) => {{
        or!($e, disjunction! ($($es),+))
    }};
}

#[macro_export]
macro_rules! pred {
    ($id: tt,$($var_id: tt),+) => {
        |builder| builder.pred($id, &[$($var_id),+])
    };
    ($id: tt) => {
        |builder| builder.pred($id, &[])
    };
}

#[macro_export]
macro_rules! every {
    ($id: tt,$e: expr) => {
        |builder| builder.every($id, $e)
    };
}

#[macro_export]
macro_rules! every_n {
    ($id: expr,$e: expr) => {
        |builder| builder.every_n($id, $e)
    };
}

#[macro_export]
macro_rules! exist {
    ($id: expr,$e: expr) => {
        exist!(id:$id,$e)
    };
    (name:$name: tt,$e: expr) => {
        |builder| {
            let id = builder.add_named(&$name.to_string());
            builder.exist(id, $e)
        }
    };
    (id:$id: expr,$e: expr) => {
        |builder| builder.exist($id, $e)
    };
}

#[macro_export]
macro_rules! exist_n {
    ($id: expr,$e: expr) => {
        exist_n!(id:$id,$e)
    };
    (name:$name: tt,$e: expr) => {{
        let id = builder.add_named(&$name.to_string());
        |builder| builder.exist_n(id, $e)
    }};
    (id:$id: expr,$e: expr) => {
        |builder| builder.exist_n($id, $e)
    };
}

#[macro_export]
macro_rules! connect {
    ($e: expr) => {
        |builder| builder.connect($e)
    };
}

#[macro_export]
macro_rules! copy {
    ($e: expr) => {
        |builder| builder.copy($e)
    };
}

#[macro_export]
macro_rules! prod {
    ($left: expr,$right: expr) => {
        |builder| builder.product($left, $right)
    };
}

#[macro_export]
macro_rules! sum {
    ($left: expr,$right: expr) => {
        |builder| builder.sum($left, $right)
    };
}
