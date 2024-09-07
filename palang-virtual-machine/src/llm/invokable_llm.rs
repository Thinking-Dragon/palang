use super::model_settings::ModelSettings;

pub trait InvokableLargeLanguageModel {
    fn invoke(
        &self,
        system: &String,
        prompt: &String,
        settings: &ModelSettings,
    ) -> impl std::future::Future<Output = Result<String, String>> + Send;
}
