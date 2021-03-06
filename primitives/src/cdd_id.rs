use crate::{IdentityId, InvestorZKProofData};
use confidential_identity::compute_cdd_id;
use polymesh_primitives_derive::SliceU8StrongTyped;

use codec::{Decode, Encode};

#[cfg(feature = "std")]
use polymesh_primitives_derive::{DeserializeU8StrongTyped, SerializeU8StrongTyped};

/// The investor UID identifies the legal entity of an investor.
/// It should be kept encrypted in order to have the investor's portfolio hidden between several
/// IdentityIds.
///
/// That UID is generated by any trusted CDD provider, based on the investor's Personal
/// Identifiable Information (PII). That process is driven by the specification of the Polymath
/// Unique Identity System (PUIS).
#[derive(
    Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, SliceU8StrongTyped,
)]
#[cfg_attr(
    feature = "std",
    derive(SerializeU8StrongTyped, DeserializeU8StrongTyped)
)]
pub struct InvestorUid([u8; 16]);

impl InvestorUid {
    /// Transform into a fixed array of bytes.
    #[inline]
    pub fn to_bytes(self) -> [u8; 16] {
        self.0
    }
}

impl From<[u8; 16]> for InvestorUid {
    fn from(s: [u8; 16]) -> Self {
        Self(s)
    }
}

impl From<[u8; 32]> for InvestorUid {
    fn from(s: [u8; 32]) -> Self {
        let mut short: [u8; 16] = Default::default();
        short.copy_from_slice(&s[..16]);
        Self(short)
    }
}

/// It links the investor UID with an specific Identity DID in a way that no one can extract that
/// investor UID from this CDD Id, and the investor can create a Zero Knowledge Proof to prove that
/// an specific DID belongs to him.
/// The main purpose of this claim is to keep the privacy of the investor using several identities
/// to handle his portfolio.
#[derive(
    Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, SliceU8StrongTyped,
)]
#[cfg_attr(
    feature = "std",
    derive(SerializeU8StrongTyped, DeserializeU8StrongTyped)
)]
pub struct CddId([u8; 32]);

impl CddId {
    /// Create a new CDD Id given the `did` and the `investor_uid`.
    /// The blind factor is generated as a `Blake2b` hash of the concatenation of the given `did`
    /// and `investor_uid`.
    pub fn new(did: IdentityId, investor_uid: InvestorUid) -> Self {
        let cdd_claim_data = InvestorZKProofData::make_cdd_claim(&did, &investor_uid);
        let raw_cdd_id = compute_cdd_id(&cdd_claim_data).compress().to_bytes();

        Self(raw_cdd_id)
    }

    /// Only the zero-filled `CddId` is considered as invalid.
    #[inline]
    pub fn is_default_cdd(&self) -> bool {
        self.0 == [0u8; 32]
    }
}

impl From<[u8; 32]> for CddId {
    #[inline]
    fn from(data: [u8; 32]) -> Self {
        Self(data)
    }
}

#[cfg(test)]
mod tests {
    use crate::{CddId, IdentityId, InvestorUid};

    #[test]
    fn cdd_id_generation() {
        let alice_id_1 = IdentityId::from(1);
        let alice_id_2 = IdentityId::from(2);
        let alice_uid = InvestorUid::from(b"alice_uid".as_ref());

        let alice_cdd_id_1 = CddId::new(alice_id_1, alice_uid);
        let alice_cdd_id_2 = CddId::new(alice_id_2, alice_uid);

        assert!(alice_id_1 != alice_id_2);
        assert!(alice_cdd_id_1 != alice_cdd_id_2);
    }
}
