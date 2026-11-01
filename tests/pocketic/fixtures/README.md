# Retired codec rejection input

`retired-operator-scope.candid` is a valid pre-Wire Candid record with service,
namespace, Cashier and payment-account fields and fixed fixture principals.
It was captured with the former historical encoder before that encoder was
removed. SHA-256: `60868216fe0630a4d3153cb8f631c10537205b086be5d12c52d1ae493d4fc01c`.

Current application and private control endpoints must reject it at codec
admission before application-field interpretation. It is deliberately independent
of current DTOs; it is not an old-image installer, compatibility reader or provider
contract. Real external provider Candid replies remain typed beside their tests.

The optional old-image drivers for retired allocation, verifier, single-uploader,
map-cell and read-session releases have been retired. Their source/evidence remains
in Git history. Current installation tests still reject foreign releases and retired
formats while comparing every retained memory byte; installed lifecycle tests still
exercise current-release restore fences and preserve outstanding obligations.
