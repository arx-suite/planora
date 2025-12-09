import Home from "@/components/features/home/homepage";
import { fetchUser } from "@/lib/api/auth";

export default async function HomePage() {
    const user = await fetchUser();

    return <Home userIn={user} />;
}
