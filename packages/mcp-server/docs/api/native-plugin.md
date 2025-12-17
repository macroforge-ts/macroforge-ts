# NativePlugin
  *The main plugin class for macro expansion with caching support. `NativePlugin` is designed to be instantiated once and reused across multiple file processing operations. It maintains a cache of expansion results keyed by filepath and version, enabling efficient incremental processing.*
 ## Constructor
 ```
const&nbsp;plugin&nbsp;=&nbsp;new&nbsp;NativePlugin();
``` ## Methods
 ### processFile()
 Process a file with version-based caching:
 ```
processFile(
&nbsp;&nbsp;filepath:&nbsp;string,
&nbsp;&nbsp;code:&nbsp;string,
&nbsp;&nbsp;options?:&nbsp;ProcessFileOptions
):&nbsp;ExpandResult
``` ```
interface&nbsp;ProcessFileOptions&nbsp;&#123;
&nbsp;&nbsp;//&nbsp;Cache&nbsp;key&nbsp;-&nbsp;if&nbsp;unchanged,&nbsp;returns&nbsp;cached&nbsp;result
&nbsp;&nbsp;version?:&nbsp;string;
&#125;
``` ### getMapper()
 Get the position mapper for a previously processed file:
 ```
getMapper(filepath:&nbsp;string):&nbsp;NativeMapper&nbsp;|&nbsp;null
``` ### mapDiagnostics()
 Map diagnostics from expanded positions to original positions:
 ```
mapDiagnostics(
&nbsp;&nbsp;filepath:&nbsp;string,
&nbsp;&nbsp;diagnostics:&nbsp;JsDiagnostic[]
):&nbsp;JsDiagnostic[]
``` ### log() / setLogFile()
 Logging utilities for debugging:
 ```
log(message:&nbsp;string):&nbsp;void
setLogFile(path:&nbsp;string):&nbsp;void
``` ## Caching Behavior
 The plugin caches expansion results by file path and version:
 ```
const&nbsp;plugin&nbsp;=&nbsp;new&nbsp;NativePlugin();

//&nbsp;First&nbsp;call&nbsp;-&nbsp;performs&nbsp;expansion
const&nbsp;result1&nbsp;=&nbsp;plugin.processFile("user.ts",&nbsp;code,&nbsp;&#123;&nbsp;version:&nbsp;"1"&nbsp;&#125;);

//&nbsp;Same&nbsp;version&nbsp;-&nbsp;returns&nbsp;cached&nbsp;result&nbsp;instantly
const&nbsp;result2&nbsp;=&nbsp;plugin.processFile("user.ts",&nbsp;code,&nbsp;&#123;&nbsp;version:&nbsp;"1"&nbsp;&#125;);

//&nbsp;Different&nbsp;version&nbsp;-&nbsp;re-expands
const&nbsp;result3&nbsp;=&nbsp;plugin.processFile("user.ts",&nbsp;newCode,&nbsp;&#123;&nbsp;version:&nbsp;"2"&nbsp;&#125;);
``` ## Example: Language Server Integration
 ```
import&nbsp;&#123;&nbsp;NativePlugin&nbsp;&#125;&nbsp;from&nbsp;"macroforge";

class&nbsp;MacroforgeLanguageService&nbsp;&#123;
&nbsp;&nbsp;private&nbsp;plugin&nbsp;=&nbsp;new&nbsp;NativePlugin();

&nbsp;&nbsp;processDocument(uri:&nbsp;string,&nbsp;content:&nbsp;string,&nbsp;version:&nbsp;number)&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Process&nbsp;with&nbsp;version-based&nbsp;caching
&nbsp;&nbsp;&nbsp;&nbsp;const&nbsp;result&nbsp;=&nbsp;this.plugin.processFile(uri,&nbsp;content,&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;version:&nbsp;String(version)
&nbsp;&nbsp;&nbsp;&nbsp;&#125;);

&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Get&nbsp;mapper&nbsp;for&nbsp;position&nbsp;translation
&nbsp;&nbsp;&nbsp;&nbsp;const&nbsp;mapper&nbsp;=&nbsp;this.plugin.getMapper(uri);

&nbsp;&nbsp;&nbsp;&nbsp;return&nbsp;&#123;&nbsp;result,&nbsp;mapper&nbsp;&#125;;
&nbsp;&nbsp;&#125;

&nbsp;&nbsp;getSemanticDiagnostics(uri:&nbsp;string,&nbsp;diagnostics:&nbsp;Diagnostic[])&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Map&nbsp;positions&nbsp;from&nbsp;expanded&nbsp;to&nbsp;original
&nbsp;&nbsp;&nbsp;&nbsp;return&nbsp;this.plugin.mapDiagnostics(uri,&nbsp;diagnostics);
&nbsp;&nbsp;&#125;
&#125;
``` ## Thread Safety
 The <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">NativePlugin</code> class is thread-safe and can be used from multiple async contexts. Each file is processed in an isolated thread with its own stack space.