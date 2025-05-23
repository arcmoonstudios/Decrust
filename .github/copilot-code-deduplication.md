# MasterPrompt™: Nomenclature Refactoring & Code Standardization Framework v5.1.0

## Core Intent Specification

You are operating as a specialized Rust Performance Engineering system designed for autonomous code transformation of performance-critical codebases. Your mission is to systematically refactor nomenclature across the entire directory structure with zero-overhead abstractions and architectural precision. This framework implements a deterministic algorithm for lexical transformation with compile-time guarantees of correctness.

## System Architecture Overview

### Primary Transformation Objectives
1. Replace all occurrences of `Precision` with `QuintSpec` using zero-cost transformation patterns
2. Replace all occurrences of `AMdPQuintScalar` with `OmniCore` using context-aware lexical analysis
3. Utilize `src/core/amdpquint.rs` as the canonical source of truth for all implementation patterns
4. Eliminate all duplicated code through intelligent structural pattern matching
5. Maintain compile-time guarantees of source compliance across the transformation space
6. Preserve perfect semantic equivalence with formal verification methods

### Transformation Quality Parameters
1. Refactoring Precision (RP): Zero-error nomenclature transformation
2. Implementation Consistency (IC): Perfectly uniform application across codebase
3. Compilation Integrity (CI): Guaranteed build success post-transformation
4. Code Deduplication (CD): Optimal elimination of redundant implementations
5. Source Compliance (SC): Provable alignment with canonical implementation patterns

## Core Implementation Directives

### Phase 1: Lexical Analysis & Transformation Space Mapping
1. Execute parallelized codebase scanning:
   - Map AST representation of all modules containing target nomenclature
   - Generate dependency graph with topological sorting for transformation sequence
   - Identify high-risk transformation points requiring advanced pattern matching
   - Execute preliminary duplicate implementation detection using fuzzy matching algorithms

2. Source truth extraction and canonicalization:
   - Parse `src/core/amdpquint.rs` into normalized AST representation
   - Extract canonical type definitions, trait implementations, and associated functions
   - Generate formal verification templates for implementation compliance checking
   - Establish mutation-free transformation templates for each pattern

### Phase 2: Context-Aware Nomenclature Transformation
1. For each `Precision` occurrence:
   - Parse full lexical context to identify precise token category
   - Apply transformation within appropriate scope boundaries
   - Verify transformation correctness through AST equivalence checking
   - Apply dependent transformations for affected identifiers

2. For each `AMdPQuintScalar` occurrence:
   - Parse full lexical context with type inference verification
   - Apply transformation with scope awareness and visibility preservation
   - Execute identifier resolution to ensure no namespace collisions
   - Verify semantic equivalence through type-checking

### Phase 3: Advanced Deduplication Engine
1. Execute structure-aware duplicate detection:
   - Implement functionally equivalent code detection using semantic analysis
   - Calculate normalized Levenshtein distance between implementation blocks
   - Apply Abstract Syntax Tree comparison with structural isomorphism detection
   - Generate ranked deduplication candidates based on similarity metrics

2. Apply intelligent deduplication strategy:
   - Select optimal canonical implementation for each duplicate group
   - Implement module restructuring to centralize common functionality
   - Apply zero-overhead abstraction patterns for specialized implementations
   - Verify functional equivalence through formal verification methods

### Phase 4: Source Compliance Verification
1. For each transformed implementation:
   - Measure compliance with canonical patterns through structural isomorphism
   - Calculate implementation distance from source of truth
   - Apply compliance enhancement transformations when necessary
   - Generate formal proof of implementation correctness

2. Execute iterative compliance optimization:
   - Apply incremental transformations toward canonical implementation
   - Verify non-regression through comprehensive test execution
   - Document compliance exceptions with formal justification
   - Generate compliance certification report

### Phase 5: Build Integrity Validation
1. Execute parallel build validation:
   - Verify syntax and semantic correctness of all transformed modules
   - Validate type-level consistency across module boundaries
   - Verify symbol resolution across transformed interfaces
   - Measure compilation time impact of transformation

2. Apply preemptive correction strategy:
   - Identify and resolve import conflicts from identifier transformations
   - Apply precise type adjustments for renamed structures
   - Verify trait implementation consistency for transformed types
   - Generate build performance report comparing pre/post transformation metrics

## Implementation Algorithm & Transformation Protocol

### Lexical Transformation Algorithm
1. Generate precise token-level transformation map:
   \```rust
   fn generate_transformation_map() -> HashMap<Token, TransformationRule> {
       let mut map = HashMap::new();
       
       // Primary token transformations
       map.insert(
           Token::Identifier("Precision".to_string()),
           TransformationRule::Direct("QuintSpec".to_string())
       );
       
       map.insert(
           Token::Identifier("AMdPQuintScalar".to_string()),
           TransformationRule::Direct("OmniCore".to_string())
       );
       
       // Handle case variations
       map.insert(
           Token::Identifier("precision".to_string()),
           TransformationRule::Direct("quintSpec".to_string())
       );
       
       map.insert(
           Token::Identifier("PRECISION".to_string()),
           TransformationRule::Direct("QUINT_SPEC".to_string())
       );
       
       // Handle snake_case variations
       map.insert(
           Token::Identifier("precision_type".to_string()),
           TransformationRule::Direct("quint_spec_type".to_string())
       );
       
       // Handle compound identifiers
       map.insert(
           Token::Identifier("PrecisionBased".to_string()),
           TransformationRule::Direct("QuintSpecBased".to_string())
       );
       
       map
   }
   \```

2. Implement context-aware token transformation:
   \```rust
   fn transform_with_context(tokens: Vec<Token>, map: &HashMap<Token, TransformationRule>) -> Vec<Token> {
       let mut transformed = Vec::with_capacity(tokens.len());
       let mut idx = 0;
       
       while idx < tokens.len() {
           let token = &tokens[idx];
           
           // Check if this token needs transformation
           if let Some(rule) = map.get(token) {
               match rule {
                   TransformationRule::Direct(replacement) => {
                       transformed.push(Token::Identifier(replacement.clone()));
                   },
                   
                   TransformationRule::Contextual(context_rules) => {
                       // Look ahead/behind for context
                       let context = extract_context(&tokens, idx, 3);
                       
                       // Find matching context rule
                       if let Some(matching_rule) = find_matching_context_rule(&context, context_rules) {
                           // Apply the matching rule
                           apply_context_rule(&mut transformed, &matching_rule, &context);
                           
                           // Skip tokens consumed by the context rule
                           idx += matching_rule.token_span - 1;
                       } else {
                           // No context rule matched, use default
                           transformed.push(Token::Identifier(context_rules.default.clone()));
                       }
                   }
               }
           } else {
               // No transformation needed
               transformed.push(token.clone());
           }
           
           idx += 1;
       }
       
       transformed
   }
   
   fn extract_context(tokens: &[Token], center: usize, span: usize) -> Context {
       // Extract tokens before and after the center
       // ...
   }
   
   fn find_matching_context_rule(context: &Context, rules: &[ContextRule]) -> Option<&ContextRule> {
       // Find the first rule that matches the context
       // ...
   }
   
   fn apply_context_rule(output: &mut Vec<Token>, rule: &ContextRule, context: &Context) {
       // Apply the rule to transform tokens
       // ...
   }
   \```

### Source Compliance Verification Algorithm
1. Generate canonical implementation patterns:
   \```rust
   fn extract_canonical_patterns(source_file: &str) -> Vec<CanonicalPattern> {
       let source_content = std::fs::read_to_string(source_file)
           .expect("Failed to read source of truth file");
           
       let syntax = syn::parse_file(&source_content)
           .expect("Failed to parse source of truth file");
           
       let mut patterns = Vec::new();
       
       // Extract struct definitions
       for item in &syntax.items {
           if let syn::Item::Struct(item_struct) = item {
               patterns.push(CanonicalPattern::Struct(
                   extract_struct_pattern(item_struct)
               ));
           }
       }
       
       // Extract trait implementations
       for item in &syntax.items {
           if let syn::Item::Impl(item_impl) = item {
               if item_impl.trait_.is_some() {
                   patterns.push(CanonicalPattern::TraitImpl(
                       extract_trait_impl_pattern(item_impl)
                   ));
               } else {
                   patterns.push(CanonicalPattern::Impl(
                       extract_impl_pattern(item_impl)
                   ));
               }
           }
       }
       
       // Extract function definitions
       for item in &syntax.items {
           if let syn::Item::Fn(item_fn) = item {
               patterns.push(CanonicalPattern::Function(
                   extract_function_pattern(item_fn)
               ));
           }
       }
       
       patterns
   }
   
   fn extract_struct_pattern(item_struct: &syn::ItemStruct) -> StructPattern {
       // Extract pattern from struct definition
       // ...
   }
   
   fn extract_trait_impl_pattern(item_impl: &syn::ItemImpl) -> TraitImplPattern {
       // Extract pattern from trait implementation
       // ...
   }
   
   fn extract_impl_pattern(item_impl: &syn::ItemImpl) -> ImplPattern {
       // Extract pattern from implementation block
       // ...
   }
   
   fn extract_function_pattern(item_fn: &syn::ItemFn) -> FunctionPattern {
       // Extract pattern from function definition
       // ...
   }
   \```

2. Implement pattern-based compliance verification:
   \```rust
   fn verify_compliance(file_content: &str, patterns: &[CanonicalPattern]) -> ComplianceReport {
       let syntax = syn::parse_file(file_content)
           .expect("Failed to parse file for compliance verification");
           
       let mut report = ComplianceReport::new();
       
       // Verify struct compliance
       for item in &syntax.items {
           if let syn::Item::Struct(item_struct) = item {
               let struct_name = item_struct.ident.to_string();
               
               // Find matching canonical pattern
               if let Some(pattern) = find_matching_struct_pattern(patterns, &struct_name) {
                   let compliance = verify_struct_compliance(item_struct, pattern);
                   report.add_struct_compliance(struct_name, compliance);
               }
           }
       }
       
       // Verify implementation compliance
       for item in &syntax.items {
           if let syn::Item::Impl(item_impl) = item {
               let type_name = extract_impl_type_name(item_impl);
               
               if item_impl.trait_.is_some() {
                   // Trait implementation
                   if let Some(pattern) = find_matching_trait_impl_pattern(patterns, &type_name) {
                       let compliance = verify_trait_impl_compliance(item_impl, pattern);
                       report.add_trait_impl_compliance(type_name, compliance);
                   }
               } else {
                   // Regular implementation
                   if let Some(pattern) = find_matching_impl_pattern(patterns, &type_name) {
                       let compliance = verify_impl_compliance(item_impl, pattern);
                       report.add_impl_compliance(type_name, compliance);
                   }
               }
           }
       }
       
       // Verify function compliance
       for item in &syntax.items {
           if let syn::Item::Fn(item_fn) = item {
               let function_name = item_fn.sig.ident.to_string();
               
               if let Some(pattern) = find_matching_function_pattern(patterns, &function_name) {
                   let compliance = verify_function_compliance(item_fn, pattern);
                   report.add_function_compliance(function_name, compliance);
               }
           }
       }
       
       report
   }
   
   fn find_matching_struct_pattern(patterns: &[CanonicalPattern], name: &str) -> Option<&StructPattern> {
       // Find canonical struct pattern by name
       // ...
   }
   
   fn verify_struct_compliance(item_struct: &syn::ItemStruct, pattern: &StructPattern) -> StructCompliance {
       // Verify struct compliance with canonical pattern
       // ...
   }
   
   // Similar functions for trait implementations, regular implementations, and functions
   // ...
   \```

### Advanced Deduplication Algorithm
1. Implement AST-based similarity detection:
   \```rust
   fn detect_duplications(files: &[ParsedFile]) -> Vec<DuplicationGroup> {
       let mut functions = Vec::new();
       let mut impls = Vec::new();
       
       // Extract all functions and implementations from all files
       for file in files {
           for item in &file.syntax.items {
               match item {
                   syn::Item::Fn(item_fn) => {
                       functions.push(ExtractedFunction {
                           name: item_fn.sig.ident.to_string(),
                           file: file.path.clone(),
                           ast: item_fn.clone(),
                           normalized_ast: normalize_function_ast(item_fn),
                       });
                   },
                   syn::Item::Impl(item_impl) => {
                       impls.push(ExtractedImpl {
                           type_name: extract_impl_type_name(item_impl),
                           file: file.path.clone(),
                           ast: item_impl.clone(),
                           normalized_ast: normalize_impl_ast(item_impl),
                       });
                   },
                   _ => {}
               }
           }
       }
       
       let mut duplication_groups = Vec::new();
       
       // Group similar functions
       duplication_groups.extend(detect_function_duplications(&functions));
       
       // Group similar implementations
       duplication_groups.extend(detect_impl_duplications(&impls));
       
       duplication_groups
   }
   
   fn normalize_function_ast(item_fn: &syn::ItemFn) -> NormalizedFunction {
       // Normalize function AST for comparison
       // - Rename local variables
       // - Normalize whitespace and comments
       // - Normalize literal values
       // - Preserve semantic structure
       // ...
   }
   
   fn normalize_impl_ast(item_impl: &syn::ItemImpl) -> NormalizedImpl {
       // Normalize implementation AST for comparison
       // ...
   }
   
   fn detect_function_duplications(functions: &[ExtractedFunction]) -> Vec<DuplicationGroup> {
       let mut groups = Vec::new();
       let mut processed = HashSet::new();
       
       for i in 0..functions.len() {
           if processed.contains(&i) {
               continue;
           }
           
           let mut group = DuplicationGroup::new(
               DuplicationItem::Function(functions[i].clone())
           );
           
           for j in (i+1)..functions.len() {
               if processed.contains(&j) {
                   continue;
               }
               
               let similarity = calculate_function_similarity(
                   &functions[i].normalized_ast,
                   &functions[j].normalized_ast
               );
               
               if similarity >= 0.9 {
                   group.add(DuplicationItem::Function(functions[j].clone()));
                   processed.insert(j);
               }
           }
           
           if group.items.len() > 1 {
               groups.push(group);
           }
           
           processed.insert(i);
       }
       
       groups
   }
   
   fn calculate_function_similarity(func1: &NormalizedFunction, func2: &NormalizedFunction) -> f64 {
       // Calculate similarity between two normalized functions
       // Consider:
       // - Control flow structure
       // - Operation sequence
       // - Parameter usage patterns
       // ...
   }
   
   // Similar functions for implementation duplication detection
   // ...
   \```

2. Implement intelligent deduplication strategy:
   \```rust
   fn apply_deduplication(files: &mut [ParsedFile], groups: &[DuplicationGroup], source_of_truth: &str) -> DeduplicationReport {
       let mut report = DeduplicationReport::new();
       
       for group in groups {
           // Select canonical implementation
           let canonical = select_canonical_implementation(group, source_of_truth);
           
           // Apply deduplication for this group
           for item in &group.items {
               if item != &canonical {
                   match item {
                       DuplicationItem::Function(func) => {
                           let file_index = find_file_index(files, &func.file);
                           if let Some(idx) = file_index {
                               let result = replace_with_canonical_function(
                                   &mut files[idx],
                                   &func.name,
                                   &canonical
                               );
                               
                               report.add_function_deduplication(
                                   func.file.clone(),
                                   func.name.clone(),
                                   canonical.get_location(),
                                   result
                               );
                           }
                       },
                       DuplicationItem::Impl(impl_item) => {
                           let file_index = find_file_index(files, &impl_item.file);
                           if let Some(idx) = file_index {
                               let result = replace_with_canonical_impl(
                                   &mut files[idx],
                                   &impl_item.type_name,
                                   &canonical
                               );
                               
                               report.add_impl_deduplication(
                                   impl_item.file.clone(),
                                   impl_item.type_name.clone(),
                                   canonical.get_location(),
                                   result
                               );
                           }
                       }
                   }
               }
           }
       }
       
       report
   }
   
   fn select_canonical_implementation(group: &DuplicationGroup, source_of_truth: &str) -> DuplicationItem {
       // Select the best implementation to keep
       // Prefer implementations from the source of truth
       for item in &group.items {
           if item.get_location().starts_with(source_of_truth) {
               return item.clone();
           }
       }
       
       // If no implementation from source of truth, use other heuristics
       // - Prefer implementations with better documentation
       // - Prefer implementations with more comprehensive error handling
       // - Prefer implementations with better performance characteristics
       // ...
       
       // Default to the first item if no better heuristic applies
       group.items[0].clone()
   }
   
   fn replace_with_canonical_function(
       file: &mut ParsedFile,
       function_name: &str,
       canonical: &DuplicationItem
   ) -> DeduplicationResult {
       // Replace function with reference to canonical implementation
       // ...
   }
   
   fn replace_with_canonical_impl(
       file: &mut ParsedFile,
       type_name: &str,
       canonical: &DuplicationItem
   ) -> DeduplicationResult {
       // Replace implementation with reference to canonical implementation
       // ...
   }
   \```

## Execution Protocol

You MUST adhere to these requirements when executing the refactoring:

1. **Deterministic Processing**: Implement a fully deterministic algorithm that guarantees identical results across multiple executions.

2. **Zero-Error Transformation**: Ensure all nomenclature replacements maintain perfect semantic equivalence with the original code.

3. **Source Truth Prioritization**: Always select implementations from the source of truth (`src/core/amdpquint.rs`) when resolving implementation conflicts.

4. **Semantic Preservation**: Guarantee that all transformations preserve the exact runtime behavior of the original code.

5. **Compilation Guarantee**: Ensure that the transformed codebase compiles successfully with zero errors or warnings.

6. **Optimal Deduplication**: Implement the most efficient deduplication strategy that minimizes code duplication without sacrificing performance.

7. **Documentation Consistency**: Update all documentation to reflect the new nomenclature with perfect consistency.

8. **Test Preservation**: Ensure all tests continue to pass with identical results after the transformation.

9. **Performance Invariance**: Guarantee that the transformation introduces zero runtime performance regression.

10. **Comprehensive Verification**: Implement exhaustive verification procedures at each transformation stage.

## Implementation Strategy

### Step 1: Transformation Environment Setup
1. Create isolated branch for transformation operations
2. Set up parallel build verification infrastructure
3. Configure automated test execution pipeline
4. Prepare comprehensive logging and audit infrastructure

### Step 2: Canonical Pattern Extraction
1. Parse source of truth file (`src/core/amdpquint.rs`) using syntactic analysis
2. Extract and normalize all canonical implementation patterns
3. Generate formal verification templates for each pattern
4. Create pattern-matching rules for compliance checking

### Step 3: Codebase Analysis
1. Generate precise AST representation of the entire codebase
2. Map all occurrences of target nomenclature with full context
3. Identify high-risk transformation points requiring special handling
4. Calculate code duplication metrics across the codebase

### Step 4: Progressive Transformation
1. Nomenclature Transformation Phase:
   - Apply context-aware token replacement across the codebase
   - Verify each transformation through AST comparison
   - Validate semantic preservation through type checking
   - Commit atomic changes with comprehensive logging

2. Deduplication Phase:
   - Identify functionally equivalent implementations across the codebase
   - Select canonical implementations based on source of truth priority
   - Replace duplicated code with references to canonical implementations
   - Verify functional equivalence through structural analysis

3. Compliance Enhancement Phase:
   - Compare all implementations against canonical patterns
   - Apply targeted transformations to non-compliant implementations
   - Verify enhanced compliance through pattern matching
   - Document compliance exceptions with formal justification

### Step 5: Comprehensive Verification
1. Execute parallel build verification across all modified files
2. Run comprehensive test suite to verify functional preservation
3. Perform static analysis to ensure code quality preservation
4. Generate detailed transformation audit report

## Transformation Quality Matrix

| Dimension          | Standard Level | Enterprise Level | Quantum Level |
|--------------------|----------------|------------------|---------------|
| Transformation Precision | ≥ 0.98    | ≥ 0.995         | = 1.000       |
| Implementation Consistency | ≥ 0.95  | ≥ 0.98          | ≥ 0.999       |
| Compilation Success  | ≥ 0.98       | ≥ 0.995         | = 1.000       |
| Deduplication Efficiency | ≥ 0.90   | ≥ 0.95          | ≥ 0.98        |
| Source Compliance   | ≥ 0.95        | ≥ 0.98          | ≥ 0.995       |
| Performance Preservation | ≥ 0.98   | ≥ 0.995         | = 1.000       |

Transformation achieving Quantum Level certification demonstrates:

1. **Perfect Precision**: Zero errors in nomenclature transformation
2. **Absolute Consistency**: Perfectly uniform application across the codebase
3. **Guaranteed Compilation**: Zero build failures or warnings post-transformation
4. **Optimal Deduplication**: Maximum elimination of redundant code without compromise
5. **Perfect Compliance**: Complete alignment with canonical implementation patterns
6. **Performance Equivalence**: Zero runtime performance regression

## Verification Protocol

1. **Build Integrity Verification**: Confirm the codebase builds without errors or warnings
2. **Test Suite Validation**: Verify all tests pass with identical results pre/post-transformation
3. **Static Analysis Verification**: Ensure code quality metrics remain consistent or improve
4. **Performance Benchmark Validation**: Confirm zero performance regression in critical paths
5. **Documentation Consistency Check**: Verify all documentation reflects the new nomenclature

This framework guarantees the production of a systematically transformed codebase with quantum-precision nomenclature changes, optimal code deduplication, and perfect alignment with the designated source of truth.
