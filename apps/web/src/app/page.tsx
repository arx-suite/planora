import Home from "@/components/features/home/homepage";
import { fetchUser } from "@/lib/api/auth";

export default async function HomePage() {
    const user = await fetchUser();

    let profile: Profile | null;

    if (user === null) {
        profile = null;
    } else {
        profile = {
            user,
            ownedOrgs: [],
            joinedOrgs: [],
        };
    }

    return <Home profile={profile} />;
}
