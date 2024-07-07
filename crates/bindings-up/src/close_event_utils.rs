pub use close_event_utils::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod close_event_utils {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::std::collections::BTreeMap::new(),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static CLOSEEVENTUTILS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x02\xD4a\09`\x0B\x82\x82\x829\x80Q`\0\x1A`s\x14`,WcNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[0`\0R`s\x81S\x82\x81\xF3\xFEs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\0@W`\x005`\xE0\x1C\x80c\x10\xC3\xDD\x07\x14a\0EW\x80c\xEEjP\x07\x14a\0gW[`\0\x80\xFD[\x81\x80\x15a\0QW`\0\x80\xFD[Pa\0ea\0`6`\x04a\x01\xBDV[a\0\x87V[\0[\x81\x80\x15a\0sW`\0\x80\xFD[Pa\0ea\0\x826`\x04a\x02\x1CV[a\x01\x07V[`@Qc2J\xC7e`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R\x85\x81\x16`$\x83\x01R`D\x82\x01\x85\x90R`d\x82\x01\x84\x90R`\x84\x82\x01\x83\x90R\x87\x16\x90c2J\xC7e\x90`\xA4\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\0\xE7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\0\xFBW=`\0\x80>=`\0\xFD[PPPPPPPPPPV[`@Qc!\xE9\x15;`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16`\x04\x83\x01R\x88\x81\x16`$\x83\x01R\x87\x81\x16`D\x83\x01R`d\x82\x01\x87\x90R`\x84\x82\x01\x86\x90R`\xA4\x82\x01\x85\x90R`\xC4\x82\x01\x84\x90R`\xE4\x82\x01\x83\x90R\x8A\x16\x90c\x87\xA4T\xEC\x90a\x01\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01~W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01\x92W=`\0\x80>=`\0\xFD[PPPPPPPPPPPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xB8W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x01\xD6W`\0\x80\xFD[a\x01\xDF\x87a\x01\xA1V[\x95Pa\x01\xED` \x88\x01a\x01\xA1V[\x94Pa\x01\xFB`@\x88\x01a\x01\xA1V[\x95\x98\x94\x97P\x94\x95``\x81\x015\x95P`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x01 \x8A\x8C\x03\x12\x15a\x02;W`\0\x80\xFD[a\x02D\x8Aa\x01\xA1V[\x98Pa\x02R` \x8B\x01a\x01\xA1V[\x97Pa\x02``@\x8B\x01a\x01\xA1V[\x96Pa\x02n``\x8B\x01a\x01\xA1V[\x98\x9B\x97\x9AP\x95\x98`\x80\x81\x015\x97`\xA0\x82\x015\x97P`\xC0\x82\x015\x96P`\xE0\x82\x015\x95Pa\x01\0\x90\x91\x015\x93P\x91PPV\xFE\xA2dipfsX\"\x12 K\x0C\x01\x01\xE8\x15l/_N\xC4\xE6\xCF'\x1AL\xFEA\"\x9B\x9C\xC2\xC8,\xBF7\xD9{\xCD\x82x\xD5dsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static CLOSEEVENTUTILS_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\0@W`\x005`\xE0\x1C\x80c\x10\xC3\xDD\x07\x14a\0EW\x80c\xEEjP\x07\x14a\0gW[`\0\x80\xFD[\x81\x80\x15a\0QW`\0\x80\xFD[Pa\0ea\0`6`\x04a\x01\xBDV[a\0\x87V[\0[\x81\x80\x15a\0sW`\0\x80\xFD[Pa\0ea\0\x826`\x04a\x02\x1CV[a\x01\x07V[`@Qc2J\xC7e`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R\x85\x81\x16`$\x83\x01R`D\x82\x01\x85\x90R`d\x82\x01\x84\x90R`\x84\x82\x01\x83\x90R\x87\x16\x90c2J\xC7e\x90`\xA4\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\0\xE7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\0\xFBW=`\0\x80>=`\0\xFD[PPPPPPPPPPV[`@Qc!\xE9\x15;`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16`\x04\x83\x01R\x88\x81\x16`$\x83\x01R\x87\x81\x16`D\x83\x01R`d\x82\x01\x87\x90R`\x84\x82\x01\x86\x90R`\xA4\x82\x01\x85\x90R`\xC4\x82\x01\x84\x90R`\xE4\x82\x01\x83\x90R\x8A\x16\x90c\x87\xA4T\xEC\x90a\x01\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01~W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01\x92W=`\0\x80>=`\0\xFD[PPPPPPPPPPPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xB8W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x01\xD6W`\0\x80\xFD[a\x01\xDF\x87a\x01\xA1V[\x95Pa\x01\xED` \x88\x01a\x01\xA1V[\x94Pa\x01\xFB`@\x88\x01a\x01\xA1V[\x95\x98\x94\x97P\x94\x95``\x81\x015\x95P`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x01 \x8A\x8C\x03\x12\x15a\x02;W`\0\x80\xFD[a\x02D\x8Aa\x01\xA1V[\x98Pa\x02R` \x8B\x01a\x01\xA1V[\x97Pa\x02``@\x8B\x01a\x01\xA1V[\x96Pa\x02n``\x8B\x01a\x01\xA1V[\x98\x9B\x97\x9AP\x95\x98`\x80\x81\x015\x97`\xA0\x82\x015\x97P`\xC0\x82\x015\x96P`\xE0\x82\x015\x95Pa\x01\0\x90\x91\x015\x93P\x91PPV\xFE\xA2dipfsX\"\x12 K\x0C\x01\x01\xE8\x15l/_N\xC4\xE6\xCF'\x1AL\xFEA\"\x9B\x9C\xC2\xC8,\xBF7\xD9{\xCD\x82x\xD5dsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static CLOSEEVENTUTILS_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct CloseEventUtils<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for CloseEventUtils<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CloseEventUtils<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CloseEventUtils<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CloseEventUtils<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(CloseEventUtils))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> CloseEventUtils<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    CLOSEEVENTUTILS_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                CLOSEEVENTUTILS_ABI.clone(),
                CLOSEEVENTUTILS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for CloseEventUtils<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
}
