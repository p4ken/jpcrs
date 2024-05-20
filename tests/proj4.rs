use proj4rs::{
    nadgrids::{catalog, Catalog},
    Proj,
};

#[test]
fn proj4() -> anyhow::Result<()> {
    let grid = ????;
    catalog::add_grid(
        "/Users/p4/Downloads/JapanGridShift-master/gsb_files/TKY2JGD.gsb".into(),
        grid,
    )?;
    Proj::from_proj_string("+proj=latlong +ellps=bessel +towgs84=-146.414,507.337,680.507 +nadgrids=/Users/p4/Downloads/JapanGridShift-master/gsb_files/TKY2JGD.gsb,null +to +proj=latlong +ellps=GRS80 +towgs84=0,0,0").unwrap();
    ok(())
}
