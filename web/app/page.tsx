import { Button } from '@/components/ui/button';
import { redirect } from 'next/navigation';

export default function Page() {
    // return <Button>Click me</Button>;
    return redirect('/api/login');
}
