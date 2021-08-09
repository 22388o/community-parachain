initSidebarItems({"constant":[["AVERAGE_ON_INITIALIZE_RATIO","We assume that ~5% of the block weight is consumed by `on_initialize` handlers. This is used to limit the maximal weight of a single extrinsic."],["DAYS",""],["HOURS",""],["MAXIMUM_BLOCK_WEIGHT","We allow for 0.5 seconds of compute with a 6 second average block time."],["MILLISECS_PER_BLOCK","This determines the average expected block time that we are targeting. Blocks will be produced at a minimum duration defined by `SLOT_DURATION`. `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked up by `pallet_aura` to implement `fn slot_duration()`."],["MINUTES",""],["NORMAL_DISPATCH_RATIO","We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used by Operational  extrinsics."],["SLOT_DURATION",""]],"mod":[["impls","Auxillary struct/enums for Statemint runtime. Taken from polkadot/runtime/common (at a21cd64) and adapted for Statemint."]],"type":[["AccountId","Some way of identifying an account on the chain. We intentionally make it equivalent to the public key of our transaction signing scheme."],["AccountIndex","The type for looking up accounts. We don’t expect more than 4 billion of them, but you never know…"],["AuraId",""],["Balance","Balance of an account."],["BlockNumber","An index to a block."],["DigestItem","Digest item type."],["Hash","A hash of some data used by the chain."],["Header","Block header type as expected by this runtime."],["Index","Index of a transaction in the chain."],["Signature","Alias to 512-bit hash when used in the context of a transaction signature on the chain."]]});