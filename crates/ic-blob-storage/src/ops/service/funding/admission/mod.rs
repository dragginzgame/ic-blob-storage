//! Authenticated current local facts; no policy decision or mutation.
use super::summary::FundingJournalSummary;
use super::{FundingIntent, FundingJournalError, Memory, StableFundingJournal, UploadContext};
use crate::model::billing::{
    FundingLimits,
    journal::{FundingIntentView, FundingJournalScope},
};

pub(crate) struct FundingAdmissionContext {
    pub summary: FundingJournalSummary,
    pub limits: FundingLimits,
    pub retained: Option<FundingIntentView>,
}
impl<M: Memory> StableFundingJournal<M> {
    pub(crate) fn admission_context(
        &self,
        execution: UploadContext,
        input: FundingIntent,
    ) -> Result<FundingAdmissionContext, FundingJournalError> {
        let scope = FundingJournalScope {
            service: input.service,
            cashier: input.cashier,
            account: input.account,
            namespace: input.namespace,
        };
        let summary = self.summary(execution, scope)?;
        // Validate exactly the same canonical request as prepare, without writes.
        Self::encode_request(input)?;
        let retained = self.lookup(execution, input)?;
        Ok(FundingAdmissionContext {
            summary,
            limits: self.config.billing().funding_limits(),
            retained,
        })
    }
}
