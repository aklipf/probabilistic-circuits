/*use crate::{and, not, or};

#[macro_export]
macro_rules! expr {
    ($var:ident) => {
        |builder| {
            let id = builder.add_named(&stringify!($var).to_string());
            builder.var(id)
        }
    };
    ($pred:ident($($vars:ident),+)) => {
        |builder| {
            let pred_id=builder.add_named(&stringify!($pred).to_string());
            let vars_id=[$(builder.add_named(&stringify!($vars).to_string())),+];
            builder.pred(pred_id, &vars_id)
        }
    };
    ($pred:ident()) => {
        |builder| {
            let pred_id=builder.add_named(&stringify!($pred).to_string());
            builder.pred(pred_id, &[])
        }
    };
    (exist($var:ident): $($e:tt)+) => {
        |builder| {
            let id = builder.add_named(&stringify!($var).to_string());
            builder.exist(id, expr!($($e)+))
        }
    };
    (exist($var:ident,$($vars:ident),+): $($e:tt)+) => {
        |builder| {
            let id = builder.add_named(&stringify!($var).to_string());
            builder.exist(id, expr!(exist($($vars),+): $($e)+))
        }
    };
    (forall($var:ident): $($e:tt)+) => {
        |builder| {
            let id = builder.add_named(&stringify!($var).to_string());
            builder.every(id, expr!($($e)+))
        }
    };
    (forall($var:ident,$($vars:ident),+): $($e:tt)+) => {
        |builder| {
            let id = builder.add_named(&stringify!($var).to_string());
            builder.every(id, expr!(forall($($vars),+): $($e)+))
        }
    };
    (($($left:tt)+) & $($right:tt)+) => {
        and!(expr!($($left)+),expr!($($right)+))
    };
    ($var:ident & $($right:tt)+) => {
        and!(expr!($var),expr!($($right)+))
    };
    ($pred:ident($($var:ident),*) & $($right:tt)+) => {
        and!(expr!($pred($($var),*)),expr!($($right)+))
    };
    (($($left:tt)+) | $($right:tt)+) => {
        or!(expr!($($left)+),expr!($($right)+))
    };
    ($var:ident | $($right:tt)+) => {
        or!(expr!($var),expr!($($right)+))
    };
    ($pred:ident($($var:ident),*) | $($right:tt)+) => {
        or!(expr!($pred($($var),*)),expr!($($right)+))
    };
    (!($($e:tt)+)) => {
        not!(expr!($($e)+))
    };
    (!$var:ident $($e:tt)+) => {
        expr!((!$var) $($e)+)
    };
    (!$var:ident) => {
        expr!(!($var))
    };
    (($($e:tt)+)) => {
        expr!($($e)+)
    };
}
*/
