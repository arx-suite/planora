"use client";

import {
    Avatar,
    AvatarFallback,
    AvatarImage,
    Badge,
    Button,
    Card,
    CardContent,
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuTrigger,
    Input,
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
    Separator,
} from "@planora/ui";
import {
    Building2,
    Edit,
    LogOut,
    MoreHorizontal,
    Search as SearchIcon,
    Trash2,
} from "lucide-react";
import { AnimatePresence, motion } from "motion/react";
import Link from "next/link";
import { useMemo, useState } from "react";
import { useAuthenticatedProfile } from "@/context/profile-context";
import { config } from "@/lib/config";

const orgsData: Org[] = [
    {
        name: "acme",
        subdomain: "acme",
        organizationId: "232-34-3131",
        ownerId: "321411312",
        plan: "Free",
    },
    {
        name: "organization-2",
        subdomain: "organization_2",
        organizationId: "232-34-3131d",
        ownerId: "321411312",
        plan: "Free",
    },
    {
        name: "organization-3",
        subdomain: "organization_3",
        organizationId: "232-34-313143",
        ownerId: "321411312",
        plan: "Free",
    },
];

function getSubdomain(subdomain: string) {
    const url = new URL(config.nextjs);
    return `${url.protocol}//${subdomain}.${url.host}`;
}

export function OrgSection() {
    const { user } = useAuthenticatedProfile();
    const [planFilter, setPlanFilter] = useState<"All" | Org["plan"]>("All");

    const [query, setQuery] = useState("");

    const allOrgs = useMemo(() => [...orgsData, ...orgsData], [orgsData]);

    const filteredOrgs = useMemo(() => {
        return allOrgs.filter((o) => {
            const q = o.name.toLowerCase().includes(query.toLowerCase());
            const p = planFilter === "All" || o.plan === planFilter;
            return q && p;
        });
    }, [allOrgs, query, planFilter]);

    return (
        <div className="max-w-4xl mx-auto py-10 px-4 space-y-8 mt-20 min-h-screen">
            <Card className="rounded-2xl backdrop-blur-md bg-background/60 border">
                <CardContent className="flex items-center gap-4 p-6">
                    <Avatar className="h-14 w-14">
                        <AvatarImage src={user.avatarUrl} />
                        <AvatarFallback>{user.username.slice(0, 2).toUpperCase()}</AvatarFallback>
                    </Avatar>
                    <div className="flex flex-col">
                        <span className="font-semibold text-lg">{user.username}</span>
                        <span className="text-sm text-muted-foreground">{user.email}</span>
                    </div>
                </CardContent>
            </Card>

            <div className="flex flex-col sm:flex-row sm:items-center gap-3">
                <div className="relative flex-1">
                    <Input
                        placeholder="Search organizations..."
                        value={query}
                        onChange={(e) => setQuery(e.target.value)}
                    />
                    <SearchIcon className="absolute right-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
                </div>

                <Select onValueChange={(v) => setPlanFilter(v as any)} defaultValue="All">
                    <SelectTrigger className="w-36">
                        <SelectValue placeholder="Plan" />
                    </SelectTrigger>
                    <SelectContent>
                        <SelectItem value="All">All plans</SelectItem>
                        <SelectItem value="Free">Free</SelectItem>
                        <SelectItem value="Pro">Pro</SelectItem>
                        <SelectItem value="Enterprise">Enterprise</SelectItem>
                    </SelectContent>
                </Select>
            </div>

            <section>
                <h3 className="text-sm font-semibold text-muted-foreground mb-3">
                    Owned Organizations
                </h3>
                <AnimatePresence>
                    <motion.div layout className="space-y-3">
                        {orgsData.length > 0 ? (
                            orgsData
                                .filter((o) =>
                                    filteredOrgs.some((f) => f.organizationId === o.organizationId),
                                )
                                .map((org) => <OrgRow key={org.organizationId} org={org} isOwned />)
                        ) : (
                            <p>No owned organizations</p>
                        )}
                    </motion.div>
                </AnimatePresence>
            </section>

            <Separator />

            <section>
                <h3 className="text-sm font-semibold text-muted-foreground mb-3">
                    Joined Organizations
                </h3>
                <AnimatePresence>
                    <motion.div layout className="space-y-3">
                        {orgsData.length > 0 ? (
                            orgsData
                                .filter((o) =>
                                    filteredOrgs.some((f) => f.organizationId === o.organizationId),
                                )
                                .map((org) => (
                                    <OrgRow key={org.organizationId} org={org} isOwned={false} />
                                ))
                        ) : (
                            <p>No joined organizations</p>
                        )}
                    </motion.div>
                </AnimatePresence>
            </section>
        </div>
    );
}

function PlanBadge({ plan }: { plan: Org["plan"] }) {
    return <Badge className="capitalize text-xs px-2 py-0.5">{plan}</Badge>;
}

function OrgRow({ org, isOwned }: { org: Org; isOwned: boolean }) {
    return (
        <motion.div
            layout
            initial={{ opacity: 0, y: 6 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -6 }}
        >
            <div className="flex items-center justify-between p-3 rounded-xl border bg-card hover:shadow-sm transition-all cursor-pointer sm:p-3">
                <div className="flex items-center gap-3 truncate">
                    <div className="h-10 w-10 rounded-lg bg-muted flex items-center justify-center">
                        <Building2 className="h-5 w-5 text-muted-foreground" />
                    </div>
                    <div className="min-w-0">
                        <div className="flex items-center gap-2">
                            <Link href={`${getSubdomain(org.subdomain)}`} target="_blank">
                                <p className="font-medium truncate">{org.name}</p>
                            </Link>
                            <PlanBadge plan={org.plan} />
                        </div>
                        <p className="text-xs text-muted-foreground truncate">
                            {isOwned ? "Owner" : "Member"}
                        </p>
                    </div>
                </div>

                <DropdownMenu>
                    <DropdownMenuTrigger asChild>
                        <Button variant="ghost" size="icon" onClick={(e) => e.stopPropagation()}>
                            <MoreHorizontal className="h-4 w-4" />
                        </Button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="end">
                        <DropdownMenuItem>
                            <Edit className="mr-2 h-4 w-4" /> Edit
                        </DropdownMenuItem>
                        {!isOwned && (
                            <DropdownMenuItem>
                                <LogOut className="mr-2 h-4 w-4" /> Leave
                            </DropdownMenuItem>
                        )}
                        <DropdownMenuItem>
                            <Trash2 className="mr-2 h-4 w-4" /> Delete
                        </DropdownMenuItem>
                    </DropdownMenuContent>
                </DropdownMenu>
            </div>
        </motion.div>
    );
}
