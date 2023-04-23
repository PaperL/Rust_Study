#![allow(unused)]

macro_rules! add {
    // first arm match add!(1,2), add!(2,3) etc
    ($a:expr,$b:expr) => {{
        $a + $b
    }}; // #define add(a,b) a+b
    // Second arm macth add!(1), add!(2) etc
    ($a:expr) => {{
        $a
    }};
}

macro_rules! add_as {
    // using a ty token type for macthing datatypes passed to maccro
    ($a:expr,$b:expr,$typ:ty) => {
        $a as $typ + $b as $typ
    };
}

macro_rules! add_2{
    ($($a:expr),*)=>{{
        0 $(+$a)*   // 0 argument is valid
    }}
}

macro_rules! add_3{
    ($a:expr)=>{$a};
    ($a:expr,$b:expr)=>{{$a+$b}};
    ($a:expr,$($b:tt)*)=>{{ // tt means recursion macro define
        $a+add_3!($($b)*)
    }}
}

macro_rules! ok_or_return{
    // match something(q,r,t,6,7,8) etc
    // compiler extracts function name and arguments. It injects the values in respective varibles.
    ($a:ident($($b:tt)*))=>{{
        match $a($($b)*) {
            Ok(value)=>value,
            Err(err)=>{
                return Err(err);
            }
        }
    }};
}

macro_rules! ok_or_return_2{
    // internal rule.
    (@error $a:ident,$($b:tt)* )=>{{
        match $a($($b)*) {
            Ok(value)=>value,
            Err(err)=>{
                return Err(err);
            }
        }
    }};
   // public rule can be called by the user.
    ($a:ident($($b:tt)*))=>{
        ok_or_return_2!(@error $a,$($b)*)
    };
}

fn some_work(i: i64, j: i64) -> Result<(i64, i64), String> {
    if i + j > 2 {
        Ok((i, j))
    } else {
        Err("error".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_and_tt() {
        assert!(add!(1, 2) == 3);
        let x = 0;
        assert!(add!(x) == 0);
        assert!(add_as!(1, 2, u8) == 3);
        assert!(add_2!(1, 2, 3, 4) == 10);
        assert!(add_3!(1, 2, 3, 4) == 10);
    }

    #[test]
    fn test_token_macro() {
        let result = (|| -> Result<(), String> {
            let rst = ok_or_return!(some_work(1, 4));
            let rst = ok_or_return!(some_work(1, 0)); // Return Err
            Ok(())
        })();
        assert!(result.is_err());
    }

    #[test]
    fn test_token_macro_2() {
        let result = (|| -> Result<(), String> {
            let rst = ok_or_return_2!(some_work(1, 4));
            let rst = ok_or_return_2!(some_work(1, 0)); // Return Err
            Ok(())
        })();
        assert!(result.is_err());
    }
}
