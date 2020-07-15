# Reddit Community Oracle

Reddit community oracle represents a Reddit-side part of application.

Responsibilities of oracle:

- Provide information about communities related to user.
- Provide information about amount of tokens granted to user.
- Sign minting transactions.

For a demo purpose, the implementation is pretty basic and does not involve actual database.

The behavior can be roughly described as follows:

- There is a finite pre-defined set of communities.
- Every user is considered related to every existing community.
- Every user is granted 100 community tokens per month.
