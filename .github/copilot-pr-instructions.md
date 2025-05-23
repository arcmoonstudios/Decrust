# Copilot⚛︎Luna Singularity Protocol v1.0.0 – PR Excellence Framework

## Core PR Directives

- Implement comprehensive change description with multi-level detail
- Apply impact analysis with cross-system dependency tracking
- Execute verification summary with evidence-based quality metrics
- Implement reviewer guidance with targeted review suggestions
- Apply enterprise readiness assessment with deployment considerations

## PR Format Specification

### Title Format

\```
<type>(<scope>): <concise description>
\```

#### Type Classification

| Type     | Purpose                                      |
|----------|----------------------------------------------|
| feat     | New feature implementation                   |
| fix      | Bug fix implementation                       |
| perf     | Performance improvement                      |
| refactor | Code restructuring without behavior change   |
| test     | Test infrastructure changes                  |
| docs     | Documentation updates                        |
| build    | Build system changes                         |
| ci       | CI configuration changes                     |
| chore    | Maintenance tasks                            |

#### Title Requirements

- Maximum 72 characters including type and scope
- Clear and concise description of the primary change
- Starts with lowercase letter after type/scope
- Specific and unambiguous wording
- Conveys significance and impact accurately
- Contains no trailing punctuation

### Description Structure

\```markdown
## Change Description

[1-3 sentence high-level summary of the changes and their purpose]

### Implementation Details

- [Bullet points describing key technical implementations]
- [Focus on what changed and how it was implemented]
- [Include architectural decisions and design patterns used]
- [Highlight complex algorithms or data structures]
- [Note any significant refactorings or restructuring]

### Motivation

[Explanation of why these changes were necessary, the problems
they solve, and the benefits they provide]

### Impact Analysis

#### Performance Impact
- [Quantitative metrics: latency, throughput, memory usage]
- [Before/after comparisons with percentage changes]
- [Load test results or benchmark data]

#### Security Considerations
- [Security implications, if any]
- [Threat modeling results]
- [Changes to attack surface]
- [Mitigations implemented]

#### Compatibility
- [API compatibility considerations]
- [Database migration requirements]
- [Client compatibility impacts]
- [Deployment sequence requirements]

### Verification Evidence

#### Testing Approach
- [Description of testing methodology]
- [Test coverage metrics: statement, branch, path]
- [Performance testing results]
- [Security testing outcomes]

#### Quality Metrics
- [Static analysis results]
- [Linting outcomes]
- [Documentation coverage]
- [Technical debt assessment]

### Reviewer Guidance

#### Critical Review Areas
- [Specific components requiring careful review]
- [Complex algorithms or implementations]
- [Security-sensitive sections]
- [Performance-critical paths]

#### Review Checklist
- [ ] Implementation correctness
- [ ] Error handling completeness
- [ ] Security considerations
- [ ] Performance characteristics
- [ ] Documentation accuracy
- [ ] Test coverage adequacy
- [ ] Deployment safety

### Related Information

- **Issues:** #123, #456
- **Documentation:** [links to relevant docs]
- **Related PRs:** #789, #101
- **External References:** [RFC links, standards, etc.]
\```

## Implementation Excellence Criteria

### Content Requirements

1. **Completeness**: PR description must cover all aspects of the change
2. **Technical Precision**: Implementation details must be accurate and specific
3. **Impact Transparency**: All downstream effects must be documented
4. **Verification Evidence**: Quality metrics must be quantitative and verifiable
5. **Reviewer Assistance**: Guidance must target specific review focus areas

### Format Requirements

1. **Structure**: Follow the prescribed format exactly
2. **Clarity**: Use precise, technical language without ambiguity
3. **Conciseness**: Maximize information density while maintaining readability
4. **Evidence**: Include qualitative and quantitative data supporting the change
5. **Traceability**: Link all related artifacts, issues, and documentation

## PR Excellence Matrix

| Dimension           | Threshold | Verification Methodology                 |
|---------------------|-----------|------------------------------------------|
| Completeness        | ≥98%      | Component coverage analysis, change scope verification |
| Technical Accuracy  | ≥99%      | Implementation verification, correctness validation |
| Impact Assessment   | ≥95%      | Cross-system analysis, dependency mapping |
| Verification Quality| ≥97%      | Evidence analysis, metric verification |
| Reviewer Guidance   | ≥90%      | Focus area specification, checklist completeness |

## QA Certification

PR descriptions must achieve Elite Level certification:
- Technical Precision: ≥0.99
- Comprehensiveness: ≥0.98
- Clarity: ≥0.99
- Evidence Quality: ≥0.97
- Guidance Value: ≥0.95