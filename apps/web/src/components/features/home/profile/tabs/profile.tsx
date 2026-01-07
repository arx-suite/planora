import { Camera, Loader2 } from "lucide-react";
import type React from "react";
import { useRef, useState } from "react";
import { toast } from "sonner";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { useAuthenticatedProfile } from "@/context/profile-context";
import { config } from "@/lib/config";
import { cn } from "@/lib/utils";

const API_UPDATE_PROFILE = `${config.api}/v1/auth/profile`;

const MAX_FILE_SIZE = 2 * 1024 * 1024;
const MIN_IMAGE_SIZE = 128;
const ALLOWED_TYPES = ["image/jpeg", "image/png", "image/webp"];

export function ProfileTab() {
    const { user } = useAuthenticatedProfile();
    const [preview, setPreview] = useState<string | null>(null);

    const [isEditing, setIsEditing] = useState(false);
    const [saving, setSaving] = useState(false);

    const [form, setForm] = useState({
        username: user.username,
        email: user.email,
    });

    const [avatarFile, setAvatarFile] = useState<File | null>(null);

    const fileInputRef = useRef<HTMLInputElement>(null);

    function startEdit() {
        setIsEditing(true);
    }

    function cancelEdit() {
        setForm({
            username: user.username,
            email: user.email,
        });
        setAvatarFile(null);
        setPreview(null);
        setIsEditing(false);
    }

    async function onAvatarSelect(e: React.ChangeEvent<HTMLInputElement>) {
        const file = e.target.files?.[0];
        if (!file) return;

        try {
            await validateAvatar(file);
            setAvatarFile(file);
            setPreview(URL.createObjectURL(file));
        } catch (err) {
            toast.error(String(err));
        }
    }

    async function saveProfile() {
        try {
            setSaving(true);

            const formData = new FormData();
            formData.append("username", form.username);
            formData.append("email", form.email);

            if (avatarFile) {
                formData.append("avatar", avatarFile);
            }

            // TODO: upload the image
            /*
            const res = await fetch(API_UPDATE_PROFILE, {
                method: "PATCH",
                body: formData,
                credentials: "include",
            });

            if (!res.ok) {
                throw new Error("Failed to update profile");
            }
            */

            toast.success("Profile updated");
            setIsEditing(false);
            setAvatarFile(null);
        } catch {
            toast.error("Failed to update profile");
        } finally {
            setSaving(false);
        }
    }

    return (
        <Card className="rounded-2xl border bg-white/4 backdrop-blur">
            <CardHeader className="flex flex-row items-center justify-between">
                <CardTitle>Profile</CardTitle>

                {!isEditing && (
                    <Button variant="ghost" onClick={startEdit}>
                        Edit profile
                    </Button>
                )}
            </CardHeader>

            <CardContent className="grid gap-6 md:grid-cols-[auto_1fr]">
                <div className="relative group w-fit">
                    <Avatar className="h-20 w-20">
                        <AvatarImage src={preview ?? user.avatarUrl} />
                        <AvatarFallback>{user.username.slice(0, 2).toUpperCase()}</AvatarFallback>
                        {isEditing && (
                            <>
                                <button
                                    type="button"
                                    onClick={() => fileInputRef.current?.click()}
                                    className={cn(
                                        "absolute inset-0 flex items-center justify-center rounded-full",
                                        "bg-black/40 opacity-0 group-hover:opacity-100 transition",
                                    )}
                                >
                                    {saving ? (
                                        <Loader2 className="h-5 w-5 animate-spin text-white" />
                                    ) : (
                                        <Camera className="h-5 w-5 text-white" />
                                    )}
                                </button>

                                <input
                                    ref={fileInputRef}
                                    type="file"
                                    accept="image/*"
                                    hidden
                                    onChange={onAvatarSelect}
                                />
                            </>
                        )}
                    </Avatar>
                </div>

                <div className="space-y-4">
                    <div className="flex flex-col gap-2">
                        <Label>Username</Label>
                        <Input
                            disabled={!isEditing}
                            value={form.username}
                            onChange={(e) =>
                                setForm({
                                    ...form,
                                    username: e.target.value,
                                })
                            }
                        />
                    </div>

                    <div className="flex flex-col gap-2">
                        <Label>Email</Label>
                        <Input
                            disabled={!isEditing}
                            value={form.email}
                            onChange={(e) =>
                                setForm({
                                    ...form,
                                    email: e.target.value,
                                })
                            }
                        />
                    </div>

                    {isEditing && (
                        <div className="flex gap-2 pt-2">
                            <Button onClick={saveProfile} disabled={saving}>
                                Save changes
                            </Button>
                            <Button variant="ghost" onClick={cancelEdit}>
                                Cancel
                            </Button>
                        </div>
                    )}
                </div>
            </CardContent>
        </Card>
    );
}

function validateAvatar(file: File): Promise<void> {
    if (!ALLOWED_TYPES.includes(file.type))
        return Promise.reject("Only JPG, PNG or WEBP images are allowed");

    if (file.size > MAX_FILE_SIZE) return Promise.reject("Image must be smaller than 2MB");

    return new Promise((resolve, reject) => {
        const img = new Image();
        const url = URL.createObjectURL(file);

        img.src = url;
        img.onload = () => {
            URL.revokeObjectURL(url);
            if (img.width < MIN_IMAGE_SIZE || img.height < MIN_IMAGE_SIZE) {
                reject("Image must be at least 128x128");
            } else {
                resolve();
            }
        };
    });
}
