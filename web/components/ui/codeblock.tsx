import { CodeHighlighter, CodeHighlighterProps } from '../code-highlighter';
import { ScrollArea, ScrollBar } from './scroll-area';
// import Copy from './chat/markdown/copy';

export function CodeBlock({ code, language }: CodeHighlighterProps) {
    return (
        <div className="my-3 relative">
            {/*<Copy content={code} className="absolute right-2 top-2 z-10" />*/}
            <ScrollArea className="rounded-md outline -outline-offset-1 outline-border">
                <CodeHighlighter code={code} language={language} />
                <ScrollBar orientation="horizontal" />
            </ScrollArea>
        </div>
    );
}
