pub trait Retriever {
    type AssetModelType;
    type ErrorType;

    fn get(&self, asset_model: &Self::AssetModelType) -> Result<String, Self::ErrorType>;
}