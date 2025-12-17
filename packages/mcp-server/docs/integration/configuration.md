# Configuration
 *Macroforge can be configured with a <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">macroforge.json</code> file in your project root.*
 ## Configuration File
 Create a <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">macroforge.json</code> file:
 ```
&#123;
&nbsp;&nbsp;"allowNativeMacros":&nbsp;true,
&nbsp;&nbsp;"macroPackages":&nbsp;[],
&nbsp;&nbsp;"keepDecorators":&nbsp;false,
&nbsp;&nbsp;"limits":&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;"maxExecutionTimeMs":&nbsp;5000,
&nbsp;&nbsp;&nbsp;&nbsp;"maxMemoryBytes":&nbsp;104857600,
&nbsp;&nbsp;&nbsp;&nbsp;"maxOutputSize":&nbsp;10485760,
&nbsp;&nbsp;&nbsp;&nbsp;"maxDiagnostics":&nbsp;100
&nbsp;&nbsp;&#125;
&#125;
``` ## Options Reference
 ### allowNativeMacros
 | Type | boolean |
| Default | true |
 Enable or disable native (Rust) macro packages. Set to <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">false</code> to only allow built-in macros.
 ### macroPackages
 | Type | string[] |
| Default | [] |
 List of npm packages that provide macros. Macroforge will look for macros in these packages.
 ```
&#123;
&nbsp;&nbsp;"macroPackages":&nbsp;[
&nbsp;&nbsp;&nbsp;&nbsp;"@my-org/custom-macros",
&nbsp;&nbsp;&nbsp;&nbsp;"community-macros"
&nbsp;&nbsp;]
&#125;
``` ### keepDecorators
 | Type | boolean |
| Default | false |
 Keep <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@derive</code> decorators in the output. Useful for debugging.
 ### limits
 Configure resource limits for macro expansion:
 ```
&#123;
&nbsp;&nbsp;"limits":&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Maximum&nbsp;time&nbsp;for&nbsp;a&nbsp;single&nbsp;macro&nbsp;expansion&nbsp;(ms)
&nbsp;&nbsp;&nbsp;&nbsp;"maxExecutionTimeMs":&nbsp;5000,

&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Maximum&nbsp;memory&nbsp;usage&nbsp;(bytes)
&nbsp;&nbsp;&nbsp;&nbsp;"maxMemoryBytes":&nbsp;104857600,&nbsp;&nbsp;//&nbsp;100MB

&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Maximum&nbsp;size&nbsp;of&nbsp;generated&nbsp;code&nbsp;(bytes)
&nbsp;&nbsp;&nbsp;&nbsp;"maxOutputSize":&nbsp;10485760,&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;10MB

&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Maximum&nbsp;number&nbsp;of&nbsp;diagnostics&nbsp;per&nbsp;file
&nbsp;&nbsp;&nbsp;&nbsp;"maxDiagnostics":&nbsp;100
&nbsp;&nbsp;&#125;
&#125;
``` ## Macro Runtime Overrides
 Override settings for specific macros:
 ```
&#123;
&nbsp;&nbsp;"macroRuntimeOverrides":&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;"@my-org/macros":&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;"maxExecutionTimeMs":&nbsp;10000
&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&nbsp;&nbsp;&#125;
&#125;
```  **Warning Be careful when increasing limits, as this could allow malicious macros to consume excessive resources. ## Environment Variables
 Some settings can be overridden with environment variables:
 | Variable | Description |
| --- | --- |
| MACROFORGE_DEBUG | Enable debug logging |
| MACROFORGE_LOG_FILE | Write logs to a file |
 ```
MACROFORGE_DEBUG=1&nbsp;npm&nbsp;run&nbsp;dev
```**