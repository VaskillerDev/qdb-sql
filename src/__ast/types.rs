//! The describe all types for AST.
use crate::__ast::types::DataType::{Bool, Int, Null, Real, Symbol, Text};
use crate::__ast::types::FuncType::{OnCreate, OnDelete, OnRead, OnUpdate};
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Less};
use std::fmt::Error;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
// data types
// example: 23 : int
/// DataType is atomic-based type for higher order instances
/// It's represent values for qdb
/// ```
/// use crate::qdb_ast::__ast::types::DataType;
/// DataType::Null; // null type
/// DataType::Int(32); // int
/// DataType::Bool(true); // bool
/// // etc.
/// ```
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
    /// Convert string value to T-type value.
    /// ```
    /// use qdb_ast::__ast::types::DataType;
    /// let value = DataType::from("32".to_string()).unwrap();
    /// assert_eq!(32,value);
    /// ```
    pub fn from<T>(raw_value: String) -> Option<T>
    where
        T: std::fmt::Display + std::str::FromStr,
    {
        let result: Result<T, T::Err> = raw_value.parse::<T>();
        match result {
            Ok(v) => Some(v),
            Err(_e) => {
                error!(
                    "Type converted for value {} as {} is not correctly.",
                    raw_value,
                    std::any::type_name::<T>()
                );
                None
            }
        }
    }

    /// Similar on 'from' method, but use it for convert string to DataType.
    /// ```
    /// use qdb_ast::__ast::types::DataType;
    /// let value = DataType::from_string("32".to_string(),"int".to_string()).unwrap();
    /// assert_eq!(DataType::Int(32),value);
    /// ```
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

    /// Create DataType instance with default value.
    /// ```
    /// use qdb_ast::__ast::types::DataType;
    /// let value = DataType::from_type_default_value("int".to_string()).unwrap();
    /// assert_eq!(DataType::Int(0),value);
    /// ```
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

    /// Create DataType::Null instance with default value.
    /// ```
    /// use qdb_ast::__ast::types::DataType;
    /// let value = DataType::from_null();
    /// assert_eq!(DataType::Null,value);
    /// ```
    pub fn from_null() -> DataType {
        DataType::Null
    }

    /// Create DataType::Bool instance with default value.
    /// ```
    /// use qdb_ast::__ast::types::DataType;
    /// let value = DataType::from_bool(true);
    /// assert_eq!(DataType::Bool(true),value);
    /// ```
    pub fn from_bool(val: bool) -> DataType {
        DataType::Bool(val)
    }

    /// Create DataType::Int instance with default value.
    /// ```
    /// use qdb_ast::__ast::types::DataType;
    /// let value = DataType::from_int(32);
    /// assert_eq!(DataType::Int(32),value);
    /// ```
    pub fn from_int(val: i64) -> DataType {
        DataType::Int(val)
    }

    /// Create DataType::Real instance with default value.
    /// ```
    /// use qdb_ast::__ast::types::DataType;
    /// let value = DataType::from_real(64.0);
    /// assert_eq!(DataType::Real(64.0),value);
    /// ```
    pub fn from_real(val: f64) -> DataType {
        DataType::Real(val)
    }

    /// Create DataType::Text instance with default value.
    /// ```
    /// use qdb_ast::__ast::types::DataType;
    /// let value = DataType::from_text("my_text".to_string());
    /// assert_eq!(DataType::Text("my_text".to_string()),value);
    /// ```
    pub fn from_text(val: String) -> DataType {
        DataType::Text(val)
    }

    /// Compare DataType's.
    /// Two instance's with diffrent type of value leads to Option::None.
    /// ```
    /// use qdb_ast::__ast::types::DataType;
    /// use std::cmp::Ordering::{Equal,Greater};
    ///
    /// let value_one = DataType::Int(32);
    /// let value_two = DataType::Real(32.0);
    /// assert_eq!(Greater,value_one.compare_with(&DataType::Int(8)).unwrap());
    /// assert_eq!(None,value_one.compare_with(&value_two));
    /// ```
    pub fn compare_with(&self, other: &Self) -> Option<Ordering> {
        let combination = (self, other);
        return match combination {
            (DataType::Null, DataType::Null) => Some(Ordering::Equal),
            (DataType::Bool(l_val), DataType::Bool(r_val)) => l_val.partial_cmp(r_val),
            (DataType::Int(l_val), DataType::Int(r_val)) => l_val.partial_cmp(r_val),
            (DataType::Real(l_val), DataType::Real(r_val)) => l_val.partial_cmp(r_val),
            (DataType::Text(l_val), DataType::Text(r_val)) => l_val.partial_cmp(r_val),
            _ => None,
        };
    }

    /// Convert DataType::Symbol instance to string.
    /// ```
    /// use qdb_ast::__ast::types::DataType;
    /// use std::cmp::Ordering::{Equal,Greater};
    ///
    /// let value_one = DataType::Int(32);
    /// let value_two = DataType::Real(32.0);
    /// assert_eq!(Greater,value_one.compare_with(&DataType::Int(8)).unwrap());
    /// assert_eq!(None,value_one.compare_with(&value_two));
    /// ```
    pub fn symbol_to_string(&self) -> core::result::Result<&String, &str> {
        return match self {
            DataType::Symbol(val) => Ok(val),
            _ => Err("DataType convert has been failed"),
        };
    }
}

impl Eq for DataType {
    fn assert_receiver_is_total_eq(&self) {
        panic!("Assert receiver is total eq")
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
// data variable - composition from data types
// example: <variable name> = 23 : int
/// DataVar is composition variable name and DataType values, in short:
/// * name - is variable String name
/// * value - is DataType value.
/// It's represent construction as variable.
/// ```
/// use qdb_ast::__ast::types::{DataVar, DataType};
/// DataVar::new("my_var".to_string(),DataType::Int(32)); // variable 'my_var' with 32 value of type int
/// ```
pub struct DataVar(String, DataType);

impl DataVar {
    /// Create new runtime varaible.
    /// ```
    /// use qdb_ast::__ast::types::{DataVar, DataType};
    /// DataVar::new("my_var".to_string(),DataType::Bool(false)); // variable 'my_var' with false value of type bool
    /// ```
    pub fn new(var_name: String, data_type: DataType) -> DataVar {
        DataVar(var_name, data_type)
    }

    /// Method for destructurization DataVar to components.
    /// ```
    /// use qdb_ast::__ast::types::{DataVar, DataType};
    /// let variable = DataVar::new("my_var".to_string(),DataType::Bool(false)); // variable 'my_var' with false value of type bool
    /// let (name,value) = variable.get();
    /// assert_eq!(&"my_var".to_string(),name);
    /// assert_eq!(&DataType::Bool(false),value);
    /// ```
    pub fn get(&self) -> (&String, &DataType) {
        let DataVar(name, val) = self;
        (name, val)
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

// function types, it's can use for Expr struct
// example: onCreate
/// Function types. It's can use for detect type for evaluation from String.
/// ```
/// use qdb_ast::__ast::types::{FuncType};
/// let func_type = FuncType::from_string("onCreate".to_string());
/// assert_eq!(FuncType::OnCreate,func_type.unwrap());
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
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
    /// Convert String to FuncType, in other case return Option::None.
    /// Not case sensitive.
    /// You can use:
    /// * "onCreate" - [FuncType::OnCreate](enum.FuncType.html#variant.OnCreate)
    /// * "onUpdate" - [FuncType::OnUpdate](enum.FuncType.html#variant.OnUpdate)
    /// * "onRead" - [FuncType::OnRead](enum.FuncType.html#variant.OnRead)
    /// * "onDelete" - [FuncType::onDelete](enum.FuncType.html#variant.onDelete)
    /// ```
    /// use qdb_ast::__ast::types::{FuncType};
    /// let func_type_one = FuncType::from_string("onCreate".to_string());
    /// let func_type_two = FuncType::from_string("oNrEaD".to_string());
    /// let func_type_three = FuncType::from_string("ONupdate".to_string());
    /// assert_eq!(FuncType::OnCreate,func_type_one.unwrap());
    /// assert_eq!(FuncType::OnRead,func_type_two.unwrap());
    /// assert_eq!(FuncType::OnUpdate,func_type_three.unwrap());
    /// ```
    pub fn from_string(func_type: String) -> Option<FuncType> {
        use crate::__ast::types_annotations::{ONCREATE, ONDELETE, ONREAD, ONUPDATE};
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

// expressions for execution operation.
// It's composition from function types and data variable
// (look enum FuncType and struct DataVar )
// example: Expr equal to onCreate(a : int, b : bool)
/// It's can use for create high order function - composition.
/// It can consits of:
/// * [FuncType](enum.FuncType.html) (required)
/// * Channel names - Vec of [DataType::Symbol](enum.DataType.html) (required)
/// * Binary Expressions - Vec of [BinaryExpr](struct.BinaryExpr.html) (optional)
/// * Variables - Vec of [DataVar](struct.DataVar.html)(optional)
/// ```
/// use qdb_ast::__ast::types::{UnaryFuncExpr, FuncType, DataVar, DataType};
/// UnaryFuncExpr::new(
/// FuncType::OnCreate,
/// vec![DataType::Symbol("my_channel".to_string())],
/// Option::None,
/// Some(vec![DataVar::new("x".to_string(),DataType::Int(16))])
/// ); // it's equivalent OnCreate(my_channel)(x : int = 16)
/// ```
#[derive(Debug)]
pub struct UnaryFuncExpr {
    func_type: FuncType,
    channel_names: Vec<DataType>,
    binary_exprs: Option<Vec<BinaryExpr>>,
    vars: Option<Vec<DataVar>>,
}

impl UnaryFuncExpr {
    /// Method for create [UnaryFuncExpr](struct.UnaryFuncExpr.html) instance.
    /// It may contain:
    /// * [FuncType](enum.FuncType.html) (required)
    /// * Channel names - Vec of [DataType::Symbol](enum.DataType.html) (required)
    /// * Binary Expressions - Vec of [BinaryExpr](struct.BinaryExpr.html) (optional)
    /// * Variables - Vec of [DataVar](struct.DataVar.html)(optional)
    /// ```
    /// use qdb_ast::__ast::types::{UnaryFuncExpr, FuncType, DataVar, DataType};
    /// UnaryFuncExpr::new(
    /// FuncType::OnCreate,
    /// vec![DataType::Symbol("my_channel".to_string())],
    /// Option::None,
    /// Some(vec![DataVar::new("x".to_string(),DataType::Int(16))])
    /// ); // it's equivalent: OnCreate(my_channel)(x : int = 16)
    /// ```
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

    /// To get func type [FuncType](enum.FuncType.html) from UnaryFuncExpr instance
    /// ```
    /// use qdb_ast::__ast::types::{UnaryFuncExpr, FuncType, DataVar, DataType};
    /// let unary_func_expr = UnaryFuncExpr::new(
    /// FuncType::OnCreate,
    /// vec![DataType::Symbol("my_channel".to_string())],
    /// Option::None,
    /// Some(vec![DataVar::new("x".to_string(),DataType::Int(16))])
    /// ); // it's equivalent: OnCreate(my_channel)(x : int = 16)
    ///
    /// assert_eq!(&FuncType::OnCreate,unary_func_expr.get_func_type())
    /// ```
    pub fn get_func_type(&self) -> &FuncType {
        &self.func_type
    }

    /// To get channel names Vec<[DataType](enum.DataType.html)> from UnaryFuncExpr instance
    /// ```
    /// use qdb_ast::__ast::types::{UnaryFuncExpr, FuncType, DataVar, DataType};
    /// let unary_func_expr = UnaryFuncExpr::new(
    /// FuncType::OnCreate,
    /// vec![DataType::Symbol("my_channel".to_string())],
    /// Option::None,
    /// Some(vec![DataVar::new("x".to_string(),DataType::Int(16))])
    /// ); // it's equivalent: OnCreate(my_channel)(x : int = 16)
    ///
    /// assert_eq!(
    /// &DataType::Symbol("my_channel".to_string()),
    /// unary_func_expr.get_channel_names().get(0).unwrap()
    /// );
    /// ```
    pub fn get_channel_names(&self) -> &Vec<DataType> {
        &self.channel_names
    }

    /// To get binary expressions Vec<[DataType](enum.DataType.html)> from UnaryFuncExpr instance
    /// ```
    /// use qdb_ast::__ast::types::{UnaryFuncExpr, FuncType, DataVar, DataType, BinaryExpr};
    /// let unary_func_expr = UnaryFuncExpr::new(
    /// FuncType::OnRead,
    /// vec![DataType::Symbol("my_channel".to_string())],
    /// Some(vec![BinaryExpr::new(DataType::Symbol("x".to_string()), DataType::Int(8),">=".to_string())]),
    /// None
    /// ); // it's equivalent: OnRead(my_channel)(x: int >= 8)
    ///
    /// assert_eq!(
    /// &BinaryExpr::new(DataType::Symbol("x".to_string()), DataType::Int(8),">=".to_string()),
    /// unary_func_expr.get_binary_exprs().as_ref().unwrap().get(0).unwrap()
    /// );
    /// ```
    pub fn get_binary_exprs(&self) -> &Option<Vec<BinaryExpr>> {
        &self.binary_exprs
    }

    /// To get variables Vec<[DataVar](struct.DataType.html)> from UnaryFuncExpr instance
    /// ```
    /// use qdb_ast::__ast::types::{UnaryFuncExpr, FuncType, DataVar, DataType};
    /// let unary_func_expr = UnaryFuncExpr::new(
    /// FuncType::OnCreate,
    /// vec![DataType::Symbol("my_channel".to_string())],
    /// Option::None,
    /// Some(vec![DataVar::new("x".to_string(),DataType::Int(16))])
    /// ); // it's equivalent: OnCreate(my_channel)(x : int = 16)
    ///
    /// assert_eq!(
    /// &DataVar::new("x".to_string(),DataType::Int(16)),
    /// unary_func_expr.get_vars().as_ref().unwrap().get(0).unwrap()
    /// );
    /// ```
    pub fn get_vars(&self) -> &Option<Vec<DataVar>> {
        &self.vars
    }
}

/// It's can use for detect a argument group for your (expr) expression
#[derive(Debug, PartialOrd, PartialEq)]
pub enum ArgumentGroup {
    FuncGroup(String),
    OtherGroup(String),
    None,
}

impl ArgumentGroup {
    /// Convert string to [ArgumentGroup](enum.ArgumentGroup.html).
    /// ```
    /// use qdb_ast::__ast::types::ArgumentGroup;
    ///
    /// assert_eq!(
    /// ArgumentGroup::FuncGroup("oncreate".to_string()),
    /// ArgumentGroup::from_string(&"onCreate".to_string())
    /// );
    ///
    /// assert_eq!(
    /// ArgumentGroup::OtherGroup("other_group".to_string()),
    /// ArgumentGroup::from_string(&"other_Group".to_string())
    /// );
    /// ```
    pub fn from_string(val: &String) -> ArgumentGroup {
        use crate::__ast::types_annotations::{ONCREATE, ONDELETE, ONREAD, ONUPDATE};
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
        use crate::__ast::types::ArgumentGroup::{FuncGroup, OtherGroup};

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


// expressions for left-hand and right-hand data types
/// It's represent logic comparison for between two [DataType](enum.DataType.html).
/// ```
/// use qdb_ast::__ast::types::{BinaryExpr, DataType};
/// let condition = BinaryExpr::new(DataType::Int(32),DataType::Int(16),">=".to_string());
/// assert_eq!(true,condition.compare().unwrap());
/// ```
#[derive(Debug, Clone,PartialEq)]
pub struct BinaryExpr(DataType, DataType, String);

impl BinaryExpr {

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
    } // todo: add AND and OR operators

    // public functions

    /// Create new [BinaryExpr](struct.BinaryExpr.html) instance.
    /// ```
    /// use qdb_ast::__ast::types::{BinaryExpr, DataType};
    /// BinaryExpr::new(DataType::Int(32),DataType::Int(16),">=".to_string());
    /// ```
    pub fn new(lterm: DataType, rterm: DataType, operator: String) -> BinaryExpr {
        BinaryExpr(lterm, rterm, operator)
    }

    /// Method for destructurization [BinaryExpr](struct.BinaryExpr.html) to components.
    /// ```
    /// use qdb_ast::__ast::types::{BinaryExpr, DataType};
    ///
    /// let binary_expr = BinaryExpr::new(DataType::Int(32),DataType::Int(16),">=".to_string());
    /// let (left_data_type,right_data_type,operator) = binary_expr.get();
    ///
    /// assert_eq!(&DataType::Int(32),left_data_type);
    /// assert_eq!(&DataType::Int(16),right_data_type);
    /// assert_eq!(">=",operator);
    /// ```
    pub fn get(&self) -> (&DataType, &DataType, &str) {
        return (&self.0, &self.1, &self.2);
    }

    /// It's represent logic comparison for between two [DataType](enum.DataType.html).
    /// ```
    /// use qdb_ast::__ast::types::{BinaryExpr, DataType};
    /// let condition = BinaryExpr::new(DataType::Int(32),DataType::Int(16),">=".to_string());
    /// assert_eq!(true,condition.compare().unwrap());
    ///
    /// let condition = BinaryExpr::new(DataType::Bool(true),DataType::Int(16),"==".to_string());
    /// assert_eq!(false,condition.compare().unwrap());
    /// ```
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
    use crate::__ast::types::{BinaryExpr, DataType, DataVar};
    use crate::__ast::util::Util;
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
        println!("{:?}", a.cmp(&b));
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
        debug_assert_eq!(Some(Ordering::Less), data_type_a.compare_with(&data_type_b));
        debug_assert_eq!(Option::None, data_type_c.compare_with(&data_type_a));
    }

    #[test]
    fn test_data_type_symbol_to_string() {
        let data_type = DataType::Symbol("my_val".to_string());
        debug_assert_eq!(Ok(&String::from("my_val")), data_type.symbol_to_string());

        let data_type = DataType::Int(64);
        debug_assert_ne!(Ok(&String::from("my_val")), data_type.symbol_to_string());
    }

    #[test]
    fn test_data_var_get() -> Result<(), ()> {
        let data_var = DataVar("my_var".to_string(), DataType::Real(32.2));
        let (name, value) = data_var.get();
        debug_assert_eq!("my_var", name);
        debug_assert_eq!(DataType::Real(32.2), *value);
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
}
