use crate::ns::*;

pub struct InterfaceImplementations<'a>(pub &'a SymbolHost);

impl<'a> InterfaceImplementations<'a> {
    pub fn verify(&mut self, implementor: &Symbol, interface: &Symbol) -> Result<Vec<InterfaceImplementationLog>, DeferVerificationError> {
        let at_package = implementor.parent().unwrap().is_package();
        let expected_visibility = if at_package { Visibility::Public } else { Visibility::Internal };

        let mut interfaces = interface.all_ascending_types(self.0);
        interfaces.push(interface.clone());

        let mut log: Vec<InterfaceImplementationLog> = vec![];

        for interface in interfaces {
            interface.throw_if_unresolved().map_err(|_| DeferVerificationError)?;

            for (name, item) in interface.prototype(self.0).borrow().iter() {
                let implementor_item = implementor.prototype(self.0).get(name);

                if implementor_item.is_some() && implementor_item.clone().unwrap().visibility() != expected_visibility {
                    log.push(InterfaceImplementationLog::WrongVisibility { name: name.clone(), expected_visibility });
                }

                if implementor_item.is_none() {
                    if item.is_virtual_property() {
                        if item.getter(self.0).is_some() && !item.getter(self.0).unwrap().is_optional_interface_method() {
                            log.push(InterfaceImplementationLog::UnimplementedGetter { name: name.clone() });
                        }
                        if item.setter(self.0).is_some() && !item.setter(self.0).unwrap().is_optional_interface_method() {
                            log.push(InterfaceImplementationLog::UnimplementedSetter { name: name.clone() });
                        }
                    } else if !item.is_optional_interface_method() {
                        log.push(InterfaceImplementationLog::UnimplementedMethod { name: name.clone() });
                    }
                // Verify accessors
                } else if item.is_virtual_property() {
                    let implementor_item = implementor_item.unwrap();
                    if !implementor_item.is_virtual_property() {
                        log.push(InterfaceImplementationLog::PropertyMustBeVirtualProperty { name: name.clone() });
                    } else {
                        // Getter
                        if implementor_item.getter(self.0).is_none() {
                            if item.getter(self.0).is_some() && !item.getter(self.0).unwrap().is_optional_interface_method() {
                                log.push(InterfaceImplementationLog::UnimplementedGetter { name: name.clone() });
                            }
                        } else if item.getter(self.0).is_some() && item.getter(self.0).unwrap().signature(self.0) != implementor_item.getter(self.0).unwrap().signature(self.0) {
                            let expected_signature = item.getter(self.0).unwrap().signature(self.0);
                            expected_signature.throw_if_unresolved().map_err(|_| DeferVerificationError)?;

                            let actual_signature = implementor_item.getter(self.0).unwrap().signature(self.0);
                            actual_signature.throw_if_unresolved().map_err(|_| DeferVerificationError)?;

                            log.push(InterfaceImplementationLog::WrongGetterSignature {
                                name: name.clone(), expected_signature,
                            });
                        }

                        // Setter
                        if implementor_item.setter(self.0).is_none() {
                            if item.setter(self.0).is_some() && !item.setter(self.0).unwrap().is_optional_interface_method() {
                                log.push(InterfaceImplementationLog::UnimplementedSetter { name: name.clone() });
                            }
                        } else if item.setter(self.0).is_some() && item.setter(self.0).unwrap().signature(self.0) != implementor_item.setter(self.0).unwrap().signature(self.0) {
                            let expected_signature = item.setter(self.0).unwrap().signature(self.0);
                            expected_signature.throw_if_unresolved().map_err(|_| DeferVerificationError)?;

                            let actual_signature = implementor_item.setter(self.0).unwrap().signature(self.0);
                            actual_signature.throw_if_unresolved().map_err(|_| DeferVerificationError)?;

                            log.push(InterfaceImplementationLog::WrongSetterSignature {
                                name: name.clone(), expected_signature,
                            });
                        }
                    }
                // Verify regular method
                } else {
                    let implementor_item = implementor_item.unwrap();
                    if !implementor_item.is_method() {
                        log.push(InterfaceImplementationLog::PropertyMustBeMethod { name: name.clone() });
                    }

                    let expected_signature = item.signature(self.0);
                    expected_signature.throw_if_unresolved().map_err(|_| DeferVerificationError)?;

                    let actual_signature = implementor_item.signature(self.0);
                    actual_signature.throw_if_unresolved().map_err(|_| DeferVerificationError)?;

                    if expected_signature != actual_signature {
                        log.push(InterfaceImplementationLog::WrongMethodSignature {
                            name: name.clone(), expected_signature,
                        });
                    }
                }
            }
        }

        Ok(log)
    }
}

pub enum InterfaceImplementationLog {
    UnimplementedMethod { name: String },
    UnimplementedGetter { name: String },
    UnimplementedSetter { name: String },
    PropertyMustBeMethod { name: String },
    PropertyMustBeVirtualProperty { name: String },
    WrongMethodSignature { name: String, expected_signature: Symbol },
    WrongGetterSignature { name: String, expected_signature: Symbol },
    WrongSetterSignature { name: String, expected_signature: Symbol },
    WrongVisibility { name: String, expected_visibility: Visibility },
}