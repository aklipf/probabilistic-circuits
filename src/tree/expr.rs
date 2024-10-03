#[macro_export]
macro_rules! propositional {
    ($expr: expr) => {
        Tree::<PropositionalLogic>::build($expr)
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
    ($pred: tt,$($var: tt),+) => {
        pred!(id:$pred,$(id:$var),+)
    };
    (id:$id: tt,ids:$slice:expr) => {
        |builder| builder.pred($id, $slice)
    };
    (id:$id: tt,$(id:$var_id: tt),+) => {
        |builder| builder.pred($id, &[$($var_id),+])
    };
    (name:$name: tt,$(name:$var_name: tt),+) => {
        |builder| {
            let pred_id=builder.add_named(&$name.to_string());
            let vars_id=[$(builder.add_named(&$var_name.to_string())),+];
            builder.pred(pred_id, &vars_id)
        }
    };
    ($pred: tt) => {
        pred!(id:pred)
    };
    (id:$id: tt) => {
        |builder| builder.pred($id, &[])
    };
    (name:$name: tt) => {
        |builder| builder.pred(builder.add_named(&$name.to_string()), &[])
    };
}

#[macro_export]
macro_rules! every {
    ($id: tt,$e: expr) => {
        every!(id:$id,$e)
    };
    (name:$name: tt,$e: expr) => {
        |builder| {
            let id = builder.add_named(&$name.to_string());
            builder.every(id, $e)
        }
    };
    (id:$id: tt,$e: expr) => {
        |builder| builder.every($id, $e)
    };
}

#[macro_export]
macro_rules! every_n {
    ($id: expr,$e: expr) => {
        every_n!(id:$id,$e)
    };
    (name:$name: tt,$e: expr) => {
        |builder| {
            let id = builder.add_named(&$name.to_string());
            builder.every_n(id, $e)
        }
    };
    (id:$id: expr,$e: expr) => {
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
