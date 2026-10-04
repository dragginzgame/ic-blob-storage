# Publication media fixtures

`warm.png` (16 × 16 RGBA, 1,001 bytes) and `cool.png` (24 × 12 RGBA,
1,102 bytes) are distinct, opaque gradient images authored for local tests.
`wide.png` (1,024 × 256 RGBA, 1,049,055 bytes) uses stored zlib blocks to
cross the 1 MiB chunk boundary, with a 479-byte partial final chunk. Its first
pixel is `[200, 30, 60, 255]`; its two chunk hashes are distinct.
They contain PNG signature, IHDR, zlib-compressed scanlines in IDAT and IEND,
with CRCs. They contain no consumer content, metadata hints or embedded code.

The native-driver journeys freeze these exact bytes with the maintained manifest
builder. Caffeine's actual SDK independently prepares the same roots and uploads
to an owned HTTPS/HTTP2 substitute. Native verification and tenant downloads check
whole-file integrity. Chromium then decodes the downloaded files and checks their
dimensions and first pixel: `[200, 30, 60, 255]` and `[20, 180, 210, 255]`.
Lost-reply recovery must complete without another upload; corruption must stop
before attestation or the next image.

The multi-chunk cases corrupt a byte after the first complete chunk, lose the
final-chunk reply and recover without another PUT. Successful cases retain a
second reference and download through it after releasing the first; releasing
both makes new descriptor requests unavailable while preserving physical and
billing-liability bytes. A fresh map must reject publication after release even
though the original retain receipt and previously completed map still exist.

These fixtures qualify local format handling, not consumer asset adoption or
deployed Caffeine MIME, CORS, cache, CSP, retention or billing behavior.
