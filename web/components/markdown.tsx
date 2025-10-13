import Markdown from 'react-markdown';
import rehypeKatex from 'rehype-katex';
import remarkGfm from 'remark-gfm';
import remarkMath from 'remark-math';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from './ui/table';
import { CodeBlock } from './ui/codeblock';

export type MarkdownProps = {
    children: string | null;
};

export function InlineCode({ children, ...props }: React.ComponentProps<'span'>) {
    return (
        <span
            className="px-1.5 py-0.5 rounded-sm bg-accent text-accent-foreground font-mono text-sm"
            {...props}
        >
            {children}
        </span>
    );
}

export default function MarkdownRenderer({ children }: MarkdownProps) {
    return (
        <Markdown
            remarkPlugins={[remarkGfm, remarkMath]}
            rehypePlugins={[rehypeKatex]}
            components={{
                h1: ({ children, ...props }) => (
                    <h1
                        className="scroll-m-20 text-4xl font-bold tracking-tight text-balance [&:not(:first-child)]:mt-6 mb-4"
                        {...props}
                    >
                        {children}
                    </h1>
                ),
                h2: ({ children, ...props }) => (
                    <h2 className="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight first:mt-0 [&:not(:first-child)]:mt-6 mb-4">
                        {children}
                    </h2>
                ),
                h3: ({ children, ...props }) => (
                    <h3
                        className="scroll-m-20 text-2xl font-semibold tracking-tight first:mt-0 [&:not(:first-child)]:mt-6 mb-3"
                        {...props}
                    >
                        {children}
                    </h3>
                ),
                p: ({ children, ...props }) => (
                    <p className="leading-7 [&:not(:first-child)]:mt-2" {...props}>
                        {children}
                    </p>
                ),
                ul: ({ children, ...props }) => (
                    <ul className="my-2 ml-3 list-disc list-inside [&>li]:mt-2" {...props}>
                        {children}
                    </ul>
                ),
                table: ({ children, ...props }) => (
                    <div className="border rounded-md my-3">
                        <Table className="max-w-full" {...props}>
                            {children}
                        </Table>
                    </div>
                ),
                hr: ({ ...props }) => <hr className="my-4" {...props} />,
                thead: ({ children, ...props }) => <TableHeader {...props}>{children}</TableHeader>,
                tbody: ({ children, ...props }) => <TableBody {...props}>{children}</TableBody>,
                th: ({ children, ...props }) => <TableHead {...props}>{children}</TableHead>,
                tr: ({ children, ...props }) => <TableRow {...props}>{children}</TableRow>,
                td: ({ children, ...props }) => <TableCell {...props}>{children}</TableCell>,
                code: ({ children, className, node, ...props }) => {
                    const isInline = node?.position?.start?.line === node?.position?.end?.line;

                    const match = /language-(\w+)/.exec(className || '');
                    const lang = match?.[1] || 'text';

                    if (isInline) {
                        return <InlineCode {...props}>{children}</InlineCode>;
                    }

                    return (
                        <div className="mt-6 mb-4">
                            <CodeBlock code={children as string} language={lang} />
                        </div>
                    );
                },
            }}
        >
            {children}
        </Markdown>
    );
}
