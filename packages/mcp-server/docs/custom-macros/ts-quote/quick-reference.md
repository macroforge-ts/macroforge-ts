## Quick Reference

| `@&#123;expr&#125;` 
            | Interpolate a Rust expression (adds space after) 
        

        
            | `&#123;| content |&#125;` 
            | Ident block: concatenates without spaces (e.g., <code
                    >&#123;|get@&#123;name&#125;|&#125;</code
                >
                → `getUser`)</td
            >
        

        
            <td>`&#123;> "comment" <&#125;` 
            | Block comment: outputs `/* comment */` (string preserves
                whitespace)</td
            >
        

        
            <td>`&#123;>> "doc" <<&#125;` 
            | Doc comment: outputs `/** doc */` (string preserves whitespace)</td
            >
        

        
            <td>`@@&#123;` 
            | Escape for literal `@&#123;` (e.g.,
                `"@@&#123;foo&#125;"`
                → `@&#123;foo&#125;`)</td
            >
        

        
            <td>`"text @&#123;expr&#125;"` 
            | String interpolation (auto-detected) 
        

        
            | `"'^template $&#123;js&#125;^'"` 
            | JS backtick template literal (outputs <code
                    >`template $&#123;js&#125;`</code
                >)</td
            >
        

        
            <td>`&#123;#if cond&#125;...&#123;/if&#125;` 
            | Conditional block 
        

        
            | <code
                    >&#123;#if cond&#125;...&#123;:else&#125;...&#123;/if&#125;</code
                ></td
            >
            <td>Conditional with else 
        

        
            | <code
                    >&#123;#if a&#125;...&#123;:else if
                    b&#125;...&#123;:else&#125;...&#123;/if&#125;</code
                ></td
            >
            <td>Full if/else-if/else chain 
        

        
            | <code
                    >&#123;#if let pattern = expr&#125;...&#123;/if&#125;</code
                ></td
            >
            <td>Pattern matching if-let 
        

        
            | <code
                    >&#123;#match expr&#125;&#123;:case
                    pattern&#125;...&#123;/match&#125;</code
                ></td
            >
            <td>Match expression with case arms 
        

        
            | <code>&#123;#for item in list&#125;...&#123;/for&#125;</code
                ></td
            >
            <td>Iterate over a collection 
        

        
            | `&#123;#while cond&#125;...&#123;/while&#125;` 
            | While loop 
        

        
            | <code
                    >&#123;#while let pattern = expr&#125;...&#123;/while&#125;</code
                ></td
            >
            <td>While-let pattern matching loop 
        

        
            | `&#123;$let name = expr&#125;` 
            | Define a local constant 
        

        
            | `&#123;$let mut name = expr&#125;` 
            | Define a mutable local variable 
        

        
            | `&#123;$do expr&#125;` 
            | Execute a side-effectful expression 
        

        
            | `&#123;$typescript stream&#125;` 
            <td
                >Inject a TsStream, preserving its source and runtime_patches
                (imports)</td
            >

**Note:** A single `@` not followed by `&#123;` passes through unchanged (e.g., `email@domain.com` works as expected).