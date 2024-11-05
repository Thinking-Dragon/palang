use palang_core::profile::Profile;

pub trait InvokableLargeLanguageModel {
    fn invoke(
        &self,
        system: &String,
        prompt: &String,
        profile: &Profile,
    ) -> impl std::future::Future<Output = Result<String, String>> + Send;
}
