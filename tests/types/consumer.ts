// A consumer's view of the published package, compiled with skipLibCheck disabled:
// every declaration file the package ships has to typecheck on its own
import { Align, Cfonts, Font, Valign } from "cfonts";

const banner = Cfonts.text("hello").font(Font.Block).align(Align.Center).valign(Valign.Middle);
const rendered = banner.renderBrowser();
const text: string = rendered.text;

console.log(text.length > 0, banner.renderCli().text, banner.renderBrowserConsole().text);
