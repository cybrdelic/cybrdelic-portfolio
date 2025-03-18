# Usage Guide

This guide explains how to use Lester for creating, claiming, and completing bounties, as well as participating in the verification process.

## Installation

Before you begin, make sure you have Lester installed on your system:

```bash
# Using cargo
cargo install lester

# Verify installation
lester --version
```

## Basic Commands

Lester uses a simple command structure:

```bash
lester <command> [subcommand] [options]
```

To get help for any command:

```bash
lester help <command>
```

## Creating a Bounty

### Basic Bounty Creation

To create a new bounty:

```bash
lester bounty create
```

This will start an interactive process to define your bounty:

1. **Title**: A concise description of the task
2. **Description**: Detailed requirements (supports Markdown)
3. **Reward**: The amount you're offering for completion
4. **Deadline**: The latest date for submissions
5. **Requirements**: Dependencies, languages, frameworks, etc.
6. **Verification Method**: How submissions will be evaluated

### Advanced Bounty Creation

For more control, use the non-interactive mode:

```bash
lester bounty create \
  --title "Implement OAuth2 Authentication" \
  --description-file requirements.md \
  --reward 200 \
  --deadline 2023-08-30 \
  --tags "rust,auth,oauth2" \
  --verification-tests ./tests
```

### Funding a Bounty

Once created, you need to fund your bounty:

```bash
lester bounty fund <bounty-id> --amount 200
```

This transfers the specified amount to the bounty escrow account.

## Finding Bounties

### Browse Available Bounties

To see all available bounties:

```bash
lester bounty list
```

This displays a paginated list of active bounties.

### Search with Filters

Search for bounties matching specific criteria:

```bash
lester bounty search --tags rust,crypto --min-reward 100
```

Available filters include:

- `--tags`: Specific technologies or categories
- `--min-reward`: Minimum reward amount
- `--max-days`: Maximum days until deadline
- `--difficulty`: Estimated difficulty level
- `--creator`: Filter by bounty creator

### View Bounty Details

To see complete details for a specific bounty:

```bash
lester bounty show <bounty-id>
```

## Working on Bounties

### Claiming a Bounty

When you find a bounty you want to work on:

```bash
lester bounty claim <bounty-id>
```

Claiming a bounty doesn't guarantee exclusivity but signals your intent to work on it.

### Asking Questions

If you need clarification on a bounty:

```bash
lester bounty ask <bounty-id> "Is feature X required?"
```

The question and answer will be publicly visible on the bounty.

### Submitting Work

When your solution is ready:

```bash
lester bounty submit <bounty-id> --path ./solution
```

The system will package your solution and initiate the verification process.

## Verification Process

### Automated Verification

For bounties with automated tests:

```bash
lester verify <submission-id> --auto
```

This runs the predefined tests against the submission. Results are automatically recorded.

### Manual Verification

For bounties requiring manual review:

```bash
lester verify <submission-id> --manual
```

This starts an interactive review process where you can:

1. Examine the submitted code
2. Run custom tests
3. Rate quality aspects
4. Provide detailed feedback

### Verification Status

Check the status of a verification:

```bash
lester verify status <submission-id>
```

## Managing Disputes

### Filing a Dispute

If you disagree with a verification result:

```bash
lester dispute create <submission-id> --reason "Tests are not aligned with requirements"
```

### Providing Evidence

Add evidence to support your dispute:

```bash
lester dispute evidence <dispute-id> --file ./evidence.md
```

### Tracking Dispute Resolution

Monitor the status of your dispute:

```bash
lester dispute status <dispute-id>
```

## Managing Your Account

### Wallet Operations

Add funds to your Lester wallet:

```bash
lester wallet deposit --amount 500
```

Withdraw funds:

```bash
lester wallet withdraw --amount 300 --to <destination>
```

### Viewing Transaction History

See your transaction history:

```bash
lester wallet history
```

### Managing Identity

Update your profile information:

```bash
lester profile update --name "Jane Doe" --bio "Rust developer"
```

Add skills to your profile:

```bash
lester profile skills add "rust,cryptography,web-development"
```

## Community Participation

### Becoming a Verifier

Register as a verifier:

```bash
lester verifier register --skills "python,javascript,react"
```

### Voting on Governance Proposals

View active proposals:

```bash
lester governance list
```

Vote on a proposal:

```bash
lester governance vote <proposal-id> --approve
```

## Advanced Features

### Local Environment Testing

Test a bounty's verification environment locally:

```bash
lester test-env <bounty-id>
```

This creates a local environment matching the verification nodes.

### Creating Bounty Templates

Save a bounty as a template for future use:

```bash
lester bounty template <bounty-id> --name "Standard Bug Fix"
```

Use a template to create a new bounty:

```bash
lester bounty create --from-template "Standard Bug Fix"
```

### Batch Operations

Process multiple bounties at once:

```bash
lester bounty batch --file operations.json
```

### Scripting and Automation

Lester supports a scripting interface for automation:

```bash
lester script run ./auto-claim.ls
```

## Tips for Success

### For Bounty Creators

1. Be specific about requirements
2. Set reasonable deadlines
3. Provide thorough test cases
4. Be responsive to questions
5. Offer fair compensation

### For Bounty Hunters

1. Read requirements carefully
2. Ask clarifying questions early
3. Test your solution thoroughly
4. Document your approach
5. Submit before deadlines

## Getting Help

Access the built-in documentation:

```bash
lester docs
```

Join the community chat:

```bash
lester community join
```

Report issues:

```bash
lester feedback --type bug --description "Issue details"
```