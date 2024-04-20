/*
Вот список причин выбора типов Box<T>, Rc<T> или RefCell<T>:

Тип Rc<T> разрешает множественное владение одними и теми же данными; типы Box<T> и RefCell<T> разрешают иметь единственных владельцев.
Тип Box<T> разрешает неизменяемые или изменяемые владения, проверенные при компиляции;
тип Rc<T> разрешает только неизменяемые владения, проверенные при компиляции;
тип RefCell<T> разрешает неизменяемые или изменяемые владения, проверенные во время выполнения.
Поскольку RefCell<T> разрешает изменяемые заимствования, проверенные во время выполнения, можно изменять значение внутри RefCell<T> даже если RefCell<T> является неизменным.
 */
pub mod box_example;
pub mod recursive_types_with_box;
pub mod box_as_link;
pub mod drop_trait;
pub mod rc_pointers;
pub mod refcell;
pub mod rc_with_refcell;