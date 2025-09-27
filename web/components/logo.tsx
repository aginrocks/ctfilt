import Link from 'next/link';
import Image from 'next/image';
import { cn } from '@lib/utils';

export type LogoProps = React.ComponentProps<'div'> & {
    href?: string;
};

export function Logo({ className, href = '/arena', ...props }: LogoProps) {
    return (
        <Link href={href}>
            <div className={cn('px-2.5 pt-2', className)} {...props}>
                <Image
                    src="/logo.svg"
                    alt="Logo"
                    width={80}
                    height={31.9}
                    className="hidden dark:block"
                />
                <Image
                    src="/logo-light.svg"
                    alt="Logo"
                    width={80}
                    height={31.9}
                    className="dark:hidden"
                />
            </div>
        </Link>
    );
}
