/*
build(BuildContext context) -> Widget
Describes the part of the user interface represented by this widget.
override

clearMaterialBanners() -> void
Removes all the materialBanners currently in queue by clearing the queue and running normal exit animation on the current materialBanner.

clearSnackBars() -> void
Removes all the snackBars currently in queue by clearing the queue and running normal exit animation on the current snackBar.

didChangeDependencies() -> void
Called when a dependency of this State object changes.
override

dispose() -> void
Called when this object is removed from the tree permanently.
override

hideCurrentMaterialBanner({MaterialBannerClosedReason reason = MaterialBannerClosedReason.hide}) -> void
Removes the current MaterialBanner by running its normal exit animation.

hideCurrentSnackBar({SnackBarClosedReason reason = SnackBarClosedReason.hide}) -> void
Removes the current SnackBar by running its normal exit animation.

removeCurrentMaterialBanner({MaterialBannerClosedReason reason = MaterialBannerClosedReason.remove}) -> void
Removes the current MaterialBanner (if any) immediately from registered Scaffolds.

removeCurrentSnackBar({SnackBarClosedReason reason = SnackBarClosedReason.remove}) -> void
Removes the current SnackBar (if any) immediately from registered Scaffolds.


showMaterialBanner(MaterialBanner materialBanner) -> ScaffoldFeatureController<MaterialBanner, MaterialBannerClosedReason>
Shows a MaterialBanner across all registered Scaffolds.

showSnackBar(SnackBar snackBar) -> ScaffoldFeatureController<SnackBar, SnackBarClosedReason>
Shows a SnackBar across all registered Scaffolds.
*/

// inherited from State<ScaffoldMessenger>
pub struct ScaffoldMessengerState;

impl Default for ScaffoldMessengerState {
    fn default() -> Self {
        Self {}
    }
}
