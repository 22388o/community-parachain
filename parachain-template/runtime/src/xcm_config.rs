use sp_std::marker::PhantomData;
use frame_support::traits::{
	fungibles::{self, Balanced, CreditOf},
	Contains, Currency, Get, Imbalance, OnUnbalanced,
};
use sp_runtime::traits::Zero;
use xcm_executor::traits::FilterAssetLocation;
use xcm::latest::{AssetId, Fungibility::Fungible, MultiAsset, MultiLocation, Junction::Parachain, Junctions::*};
use pallet_asset_tx_payment::HandleCredit;

/// Type alias to conveniently refer to `frame_system`'s `Config::AccountId`.
pub type AccountIdOf<R> = <R as frame_system::Config>::AccountId;

/// Asset filter that allows all assets from a certain location.
pub struct AssetsFrom<T>(PhantomData<T>);
impl<T: Get<MultiLocation>> FilterAssetLocation for AssetsFrom<T> {
	fn filter_asset_location(asset: &MultiAsset, origin: &MultiLocation) -> bool {
		let loc = T::get();
		&loc == origin &&
			matches!(asset, MultiAsset { id: AssetId::Concrete(asset_loc), fun: Fungible(_a) }
			if asset_loc.match_and_split(&loc).is_some())
	}
}

/// Allow checking in assets that have issuance > 0.
pub struct NonZeroIssuance<AccountId, Assets>(PhantomData<(AccountId, Assets)>);
impl<AccountId, Assets> Contains<<Assets as fungibles::Inspect<AccountId>>::AssetId>
	for NonZeroIssuance<AccountId, Assets>
where
	Assets: fungibles::Inspect<AccountId>,
{
	fn contains(id: &<Assets as fungibles::Inspect<AccountId>>::AssetId) -> bool {
		!Assets::total_issuance(*id).is_zero()
	}
}

/// A `HandleCredit` implementation that naively transfers the fees to the block author.
/// Will drop and burn the assets in case the transfer fails.
pub struct AssetsToBlockAuthor<R>(PhantomData<R>);
impl<R> HandleCredit<AccountIdOf<R>, pallet_assets::Pallet<R>> for AssetsToBlockAuthor<R>
where
	R: pallet_authorship::Config + pallet_assets::Config,
	AccountIdOf<R>:
		From<polkadot_primitives::v1::AccountId> + Into<polkadot_primitives::v1::AccountId>,
{
	fn handle_credit(credit: CreditOf<AccountIdOf<R>, pallet_assets::Pallet<R>>) {
		if let Some(author) = pallet_authorship::Pallet::<R>::author() {
			// In case of error: Will drop the result triggering the `OnDrop` of the imbalance.
			let _ = pallet_assets::Pallet::<R>::resolve(&author, credit);
		}
	}
}

pub trait Reserve {
	/// Returns assets reserve location.
	fn reserve(&self) -> Option<MultiLocation>;
}

// Takes the chain part of a MultiAsset
impl Reserve for MultiAsset {
	fn reserve(&self) -> Option<MultiLocation> {
		if let AssetId::Concrete(location) = self.id.clone() {
			let first_interior = location.first_interior();
			let parents = location.parent_count();
			match (parents, first_interior.clone()) {
				(0, Some(Parachain(id))) => Some(MultiLocation::new(0, X1(Parachain(id.clone())))),
				(1, Some(Parachain(id))) => Some(MultiLocation::new(1, X1(Parachain(id.clone())))),
				(1, _) => Some(MultiLocation::parent()),
				_ => None,
			}
		} else {
			None
		}
	}
}

/// A `FilterAssetLocation` implementation. Filters multi native assets whose
/// reserve is same with `origin`.
pub struct MultiNativeAsset;
impl FilterAssetLocation for MultiNativeAsset {
	fn filter_asset_location(asset: &MultiAsset, origin: &MultiLocation) -> bool {
		if let Some(ref reserve) = asset.reserve() {
			if reserve == origin {
				return true;
			}
		}
		false
	}
}
