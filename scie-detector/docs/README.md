# Framework Detector

## IDEA Implementation

```java
<? extends FacetBasedFrameworkDetector>
```
| Type        | File  | Condition |
|-------------|-------|-----------|
| AndroidFrameworkDetector  | "AndroidManifest.xml" |       |
| AppEngineFrameworkDetector | "appengine-web.xml" | xmlWithRootTag("appengine-web-app") |
| FacetBasedFrameworkDetector |  |  |


## VSCode

`workspaceTagsService.ts`

```typescript
tags['workspace.grunt'] = nameSet.has('gruntfile.js');
tags['workspace.gulp'] = nameSet.has('gulpfile.js');
tags['workspace.jake'] = nameSet.has('jakefile.js');

tags['workspace.tsconfig'] = nameSet.has('tsconfig.json');
tags['workspace.jsconfig'] = nameSet.has('jsconfig.json');
tags['workspace.config.xml'] = nameSet.has('config.xml');
tags['workspace.vsc.extension'] = nameSet.has('vsc-extension-quickstart.md');

tags['workspace.ASP5'] = nameSet.has('project.json') && this.searchArray(names, /^.+\.cs$/i);
tags['workspace.sln'] = this.searchArray(names, /^.+\.sln$|^.+\.csproj$/i);
tags['workspace.unity'] = nameSet.has('assets') && nameSet.has('library') && nameSet.has('projectsettings');
tags['workspace.npm'] = nameSet.has('package.json') || nameSet.has('node_modules');
tags['workspace.bower'] = nameSet.has('bower.json') || nameSet.has('bower_components');

tags['workspace.java.pom'] = nameSet.has('pom.xml');

tags['workspace.yeoman.code.ext'] = nameSet.has('vsc-extension-quickstart.md');

tags['workspace.py.requirements'] = nameSet.has('requirements.txt');
tags['workspace.py.requirements.star'] = this.searchArray(names, /^(.*)requirements(.*)\.txt$/i);
tags['workspace.py.Pipfile'] = nameSet.has('pipfile');
tags['workspace.py.conda'] = this.searchArray(names, /^environment(\.yml$|\.yaml$)/i);

const mainActivity = nameSet.has('mainactivity.cs') || nameSet.has('mainactivity.fs');
const appDelegate = nameSet.has('appdelegate.cs') || nameSet.has('appdelegate.fs');
const androidManifest = nameSet.has('androidmanifest.xml');

const platforms = nameSet.has('platforms');
const plugins = nameSet.has('plugins');
const www = nameSet.has('www');
const properties = nameSet.has('properties');
const resources = nameSet.has('resources');
const jni = nameSet.has('jni');

if (tags['workspace.config.xml'] &&
    !tags['workspace.language.cs'] && !tags['workspace.language.vb'] && !tags['workspace.language.aspx']) {
    if (platforms && plugins && www) {
        tags['workspace.cordova.high'] = true;
    } else {
        tags['workspace.cordova.low'] = true;
    }
}

if (tags['workspace.config.xml'] &&
    !tags['workspace.language.cs'] && !tags['workspace.language.vb'] && !tags['workspace.language.aspx']) {

    if (nameSet.has('ionic.config.json')) {
        tags['workspace.ionic'] = true;
    }
}

if (mainActivity && properties && resources) {
    tags['workspace.xamarin.android'] = true;
}

if (appDelegate && resources) {
    tags['workspace.xamarin.ios'] = true;
}

if (androidManifest && jni) {
    tags['workspace.android.cpp'] = true;
}
```
