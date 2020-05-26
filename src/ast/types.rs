//! The describe all types for ast.
use crate::ast::types::FuncType::{OnCreate, OnDelete, OnRead, OnUpdate};
use std::cmp::Ordering;
use std::cmp::Ordering::{Less, Equal};
use crate::ast::types::DataType::{Bool, Null, Real, Int, Text, Symbol};

#[derive(Debug, PartialEq, PartialOrd,Clone)]
// data types
// example: 23 : int
pub enum DataType {
    // null value
    Null,
    // bool value
    Bool(bool),
    // integer value
    Int(i64),
    // real value
    Real(f64),
    // text value
    Text(String),
    // shadow value
    Symbol(String),
}

impl Ord for DataType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl DataType {
    pub fn from<T>(raw_value: String) -> Option<T>
    where
        T: std::fmt::Display + std::str::FromStr,
    {
        let result: Result<T, T::Err> = raw_value.parse::<T>();
        match result {
            Ok(v) => Some(v),
            Err(_e) => {
                /* let mes = format!(
                    "Type converted for value {} as {} is not correctly.",
                    raw_value,
                    std::any::type_name::<T>()
                );*/
                error!(
                    "Type converted for value {} as {} is not correctly.",
                    raw_value,
                    std::any::type_name::<T>()
                );
                None
            }
        }
    }

    pub fn from_string<T: ToString>(raw_value: T, raw_type: T) -> Option<DataType> {
        use super::types::DataType::*;
        use super::types_annotations::{BOOL, INT, NULL, REAL, SYMBOL, TEXT};

        let raw_value = raw_value.to_string().to_lowercase();
        let raw_type = raw_type.to_string().to_lowercase();

        match raw_type.as_str() {
            NULL => Some(Null),
            BOOL => Some(Bool(Self::from::<bool>(raw_value)?)),
            INT => Some(Int(Self::from::<i64>(raw_value)?)),
            REAL => Some(Real(Self::from::<f64>(raw_value)?)),
            TEXT => Some(Text(raw_value)),
            SYMBOL => Some(Symbol(raw_value)),
            _ => None,
        }
    }

    pub fn from_type_default_value<T: ToString>(raw_type: T) -> Option<DataType> {
        use super::types::DataType::*;
        use super::types_annotations::{BOOL, INT, NULL, REAL, TEXT};

        let raw_type = raw_type.to_string().to_lowercase();

        match raw_type.as_str() {
            NULL => Some(Null),
            BOOL => Some(Bool(false)),
            INT => Some(Int(0)),
            REAL => Some(Real(0.0)),
            TEXT => Some(Text("".to_string())),
            _ => None,
        }
    }

    pub fn null() -> DataType {
        DataType::Null
    }

    pub fn from_bool(val: bool) -> DataType {
        DataType::Bool(val)
    }

    pub fn from_int(val: i64) -> DataType {
        DataType::Int(val)
    }

    pub fn from_real(val: f64) -> DataType {
        DataType::Real(val)
    }

    pub fn from_text(val: String) -> DataType {
        DataType::Text(val)
    }

    pub fn compare_with(&self,other: &Self) -> Option<Ordering> {
        let combination = (self,other);
        return match combination  {
            (DataType::Null,DataType::Null) => Some(Ordering::Equal),
            (DataType::Bool(l_val),DataType::Bool(r_val)) => l_val.partial_cmp(r_val),
            (DataType::Int(l_val),DataType::Int(r_val)) => l_val.partial_cmp(r_val),
            (DataType::Real(l_val),DataType::Real(r_val)) => l_val.partial_cmp(r_val),
            (DataType::Text(l_val),DataType::Text(r_val)) => l_val.partial_cmp(r_val),
            _ => None
        }
    }
}

impl Eq for DataType {
    fn assert_receiver_is_total_eq(&self) {
        panic!("Assert receiver is total eq")
    }
}

#[derive(Debug,PartialEq,PartialOrd)]
// data variable - composition from data types
// example: <variable name> = 23 : int
pub struct DataVar(String, DataType);

impl DataVar {
    pub fn new(var_name: String, data_type: DataType) -> DataVar {
        DataVar(var_name, data_type)
    }
    pub fn get(&self) -> (&String,&DataType){
        let DataVar(name,val) = self;
        (name,val)
    }
}

impl Ord for DataVar {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for DataVar {
    fn assert_receiver_is_total_eq(&self) {
        panic!("Assert receiver is total eq")
    }
}

impl std::fmt::Display for DataVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "name: {}, type: {:?}", self.0, self.1)
    }
}

#[derive(Debug, Copy, Clone)]
// function types, it's can use for Expr struct
// example: onCreate
pub enum FuncType {
    // onCreate value
    OnCreate,
    // onRead value
    OnRead,
    // onUpdate
    OnUpdate,
    // onDelete
    OnDelete,
}

impl FuncType {
    pub fn from_string(func_type: String) -> Option<FuncType> {
        use crate::ast::types_annotations::{ONCREATE, ONDELETE, ONREAD, ONUPDATE};
        let raw_type = func_type.to_string().to_lowercase();

        match raw_type.as_str() {
            // for create channel
            ONCREATE => Some(OnCreate),
            //for read channel
            ONREAD => Some(OnRead),
            // for update node in channel
            ONUPDATE => Some(OnUpdate),
            // for delete node from channel
            ONDELETE => Some(OnDelete),
            _ => None,
        }
    }
}

#[derive(Debug)]
// expressions for execution operation.
// It's composition from function types and data variable
// (look enum FuncType and struct DataVar )
// example: Expr equal to onCreate(a : int, b : bool)
pub struct UnaryFuncExpr {
    func_type: FuncType,
    channel_names: Vec<DataType>,
    binary_exprs: Option<Vec<BinaryExpr>>,
    vars: Option<Vec<DataVar>>,
}

impl UnaryFuncExpr {
    pub fn new(
        func_type: FuncType,
        channel_names: Vec<DataType>,
        binary_exprs: Option<Vec<BinaryExpr>>,
        vars: Option<Vec<DataVar>>,
    ) -> UnaryFuncExpr {
        UnaryFuncExpr {
            func_type,
            channel_names,
            binary_exprs,
            vars,
        }
    }
    pub fn get_func_type(&self) -> &FuncType {
        &self.func_type
    }
    pub fn get_channel_names(&self) -> &Vec<DataType> {
        &self.channel_names
    }
    pub fn get_binary_exprs(&self) -> &Option<Vec<BinaryExpr>> {
        &self.binary_exprs
    }
    pub fn get_vars(&self) -> &Option<Vec<DataVar>> {
        &self.vars
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum ArgumentGroup {
    FuncGroup(String),
    /* unused: */
    //ChannelsGroup(String),
    //ExpressionsGroup(String),
    //StatementsGroup(String),
    OtherGroup(String),
    None,
}

impl ArgumentGroup {
    pub fn from_string(val: &String) -> ArgumentGroup {
        use crate::ast::types_annotations::{ONCREATE, ONDELETE, ONREAD, ONUPDATE};
        let val = val.to_lowercase();
        let val = val.as_str();
        match val {
            ONCREATE | ONREAD | ONUPDATE | ONDELETE => ArgumentGroup::FuncGroup(val.to_string()),
            _ => ArgumentGroup::OtherGroup(val.to_string()),
        }
    }
}

impl ToString for ArgumentGroup {
    fn to_string(&self) -> String {
        use crate::ast::types::ArgumentGroup::{FuncGroup, OtherGroup};

        match self {
            FuncGroup(val) => val.to_owned(),
            OtherGroup(val) => val.to_owned(),
            _ => "".to_owned(),
        }
    }
}

impl std::fmt::Display for UnaryFuncExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "function type: {:?} \n\
            channel names: {:?} \n\
            binary expressions: {:?} \n\
            vars: {:?}",
            &self.func_type, &self.channel_names, &self.binary_exprs, &self.vars
        )
    }
}

#[derive(Debug,Clone)]
// expressions for left-hand and right-hand data types
pub struct BinaryExpr(DataType, DataType, String);

impl BinaryExpr {
    pub fn new(lterm: DataType, rterm: DataType, operator: String) -> BinaryExpr {
        BinaryExpr(lterm, rterm, operator)
    }

    pub fn get(&self) -> (&DataType,&DataType,&str) {
        return (&self.0,&self.1,&self.2)
    }

    fn eq(&self) -> bool {
        self.0 == self.1
    }
    fn neq(&self) -> bool {
        self.0 != self.1
    }
    fn ge(&self) -> bool {
        self.0 >= self.1
    }
    fn gt(&self) -> bool {
        self.0 > self.1
    }
    fn le(&self) -> bool {
        self.0 <= self.1
    }
    fn lt(&self) -> bool {
        self.0 < self.1
    }
    // todo: add AND and OR operators

    pub fn compare(&self) -> Option<bool> {
        match self.2.as_str() {
            "==" => Some(self.eq()),
            "!=" => Some(self.neq()),
            ">=" => Some(self.ge()),
            ">" => Some(self.gt()),
            "<=" => Some(self.le()),
            "<" => Some(self.lt()),
            _ => None,
        }
    }
}

#[cfg(test)]
// test module
mod test {
    use crate::ast::types::{BinaryExpr, DataType, DataVar};
    use crate::ast::util::Util;
    use std::cmp::Ordering;

    #[test]
    fn test_data_type_from_string() -> Result<(), ()> {
        debug_assert_eq!(
            DataType::Null,
            DataType::from_string("NULL", "null").unwrap()
        );
        debug_assert_eq!(
            DataType::Bool(true),
            DataType::from_string("true", "bool").unwrap()
        );
        debug_assert_eq!(
            DataType::Int(32),
            DataType::from_string("32", "int").unwrap()
        );
        debug_assert_eq!(
            DataType::Real(64.01),
            DataType::from_string("64.01", "real").unwrap()
        );
        debug_assert_eq!(
            DataType::Real(32.0001),
            DataType::from_string("32.0001", "real").unwrap()
        );
        debug_assert_eq!(
            DataType::Text("my test text".to_string()),
            DataType::from_string("my test text", "text").unwrap()
        );
        DataType::from_string("tru", "bool"); // todo: continue

        Ok(())
    }

    #[test]
    fn test_ordering_data_type() {
        let a = DataType::Text("0".to_string());
        let b = DataType::Int(0);
        println!("{:?}",a.cmp(&b));
    }

    #[test]
    fn test_data_type_clone() {
        let a = &DataType::Bool(true);
        let b = a.clone();
    }

    #[test]
    fn test_data_type_compare_with() {
        let data_type_a = DataType::Int(32);
        let data_type_b = DataType::Int(64);
        let data_type_c = DataType::Real(32.0);
        let data_type_d = DataType::Text("my test text".to_string());
        let data_type_e = DataType::Bool(true);

        // todo: continue
        debug_assert_eq!(Some(Ordering::Less),data_type_a.compare_with(&data_type_b));
        debug_assert_eq!(Option::None, data_type_c.compare_with(&data_type_a));
    }

    #[test]
    fn test_data_var_get() -> Result<(),()> {
        let data_var = DataVar("my_var".to_string(),DataType::Real(32.2));
        let (name,value) = data_var.get();
        debug_assert_eq!("my_var",name);
        debug_assert_eq!(DataType::Real(32.2),*value);
        Ok(())
    }

    #[test]
    fn test_is_single_word() -> Result<(), ()> {
        debug_assert_eq!(true, Util::is_single_word("myvarexample".to_string()));
        debug_assert_eq!(true, Util::is_single_word("myvar23varmy".to_string()));
        debug_assert_eq!(true, Util::is_single_word("m".to_string()));
        debug_assert_eq!(true, Util::is_single_word("  myvar23varmy".to_string()));

        debug_assert_eq!(false, Util::is_single_word("5".to_string()));
        debug_assert_eq!(
            false,
            Util::is_single_word("myvar exa23mple text".to_string())
        );
        debug_assert_eq!(false, Util::is_single_word("2123example".to_string()));
        Ok(())
    }

    #[test]
    fn test_identify_type() -> Result<(), ()> {
        debug_assert_eq!("null", Util::identify_type(&"null".to_string()));
        debug_assert_eq!("text", Util::identify_type(&"'my string text'".to_string()));
        debug_assert_eq!("int", Util::identify_type(&"28".to_string()));
        debug_assert_eq!("symbol", Util::identify_type(&"my_var".to_string()));
        debug_assert_eq!("real", Util::identify_type(&"32.0".to_string()));
        Ok(())
    }

    #[test]
    fn test_binary_expr_compare() -> Result<(), ()> {
        debug_assert_eq!(
            true,
            BinaryExpr(
                DataType::Text("my text".to_string()),
                DataType::Text("my text".to_string()),
                "==".to_string()
            )
            .compare()
            .unwrap()
        );

        debug_assert_eq!(
            true,
            BinaryExpr(
                DataType::Text("my text double".to_string()),
                DataType::Text("my text".to_string()),
                ">=".to_string()
            )
            .compare()
            .unwrap()
        );

        debug_assert_eq!(
            true,
            BinaryExpr(
                DataType::Text("my text".to_string()),
                DataType::Text("my text double".to_string()),
                "<=".to_string()
            )
            .compare()
            .unwrap()
        );

        debug_assert_eq!(
            true,
            BinaryExpr(DataType::Int(32), DataType::Real(32.0), "!=".to_string())
                .compare()
                .unwrap()
        );

        debug_assert_eq!(
            true,
            BinaryExpr(DataType::Null, DataType::Null, "==".to_string())
                .compare()
                .unwrap()
        );

        debug_assert_eq!(
            false,
            BinaryExpr(DataType::Bool(true), DataType::Null, "==".to_string())
                .compare()
                .unwrap()
        );

        debug_assert_eq!(
            false,
            BinaryExpr(DataType::Int(32), DataType::Real(32.0), "==".to_string())
                .compare()
                .unwrap()
        );

        Ok(())
    }

    #[test]
    fn test_binary_expr_get() {
        let binary_expr = BinaryExpr::new(DataType::Symbol("S".to_string()),DataType::Int(32),"==".to_string());
        println!("{:#?}",binary_expr.get())
    }
}
