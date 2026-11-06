# Security policy

## Supported versions

Only the latest 0.1.x release receives fixes.

## Reporting a vulnerability

Please do not open a public issue. Report it privately through GitHub:
[Report a vulnerability](https://github.com/robmeijerink/snelstart-b2b/security/advisories/new).

Include the version of the crate and, if you can, a few lines of code that reproduce the problem.
Do not send real client keys, subscription keys, access tokens or administration data.

## What is in scope

This crate handles the SnelStart client key, the subscription key and the access tokens obtained
with them, so these are worth reporting:

- a client key, subscription key or access token that ends up in a log, an error or `Debug` output;
- a request that goes to another host than intended, or one that follows a redirect;
- a token that is cached, reused or sent when it should not be (for example after it has expired
  or was rejected);
- an id or value that passes validation but should not, or that changes silently on the way to the
  API (for example a path segment or OData filter that escapes its place in the URL);
- a panic or unbounded memory use caused by a crafted response.

## What is not in scope

- Problems in the SnelStart API or platform itself. Report those to SnelStart.
- Behaviour that is documented: errors can contain a part of a response body, which may hold
  administration data, and a response has no size limit. The copy of the keys and tokens inside a
  request header is held by the HTTP client and is not wiped from memory.
