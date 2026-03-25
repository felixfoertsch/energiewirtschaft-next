import { Badge } from "@/components/ui/badge.tsx";
import { Tabs, TabsList, TabsTrigger } from "@/components/ui/tabs.tsx";
import { rollenLabel } from "@/lib/rollen.ts";
import type { Rolle } from "@/lib/types.ts";

interface RollenTabsProps {
	rollen: Rolle[];
	aktiveRolle: string;
	onRolleChange: (rolle: string) => void;
	unreadCounts?: Record<string, number>;
}

export function RollenTabs({ rollen, aktiveRolle, onRolleChange, unreadCounts }: RollenTabsProps) {
	return (
		<Tabs value={aktiveRolle} onValueChange={onRolleChange}>
			<div className="border-b px-6">
				<TabsList className="h-auto gap-2 bg-transparent p-0">
					{rollen.map((r) => {
						const count = unreadCounts?.[r.name] ?? 0;
						return (
							<TabsTrigger
								key={r.name}
								value={r.name}
								className="rounded-none border-b-2 border-transparent px-3 py-2 data-[state=active]:border-primary data-[state=active]:bg-transparent"
							>
								<span>{rollenLabel(r.name)}</span>
								{count > 0 && (
									<Badge variant="secondary" className="ml-1.5 h-5 min-w-5 px-1 text-[10px]">
										{count}
									</Badge>
								)}
							</TabsTrigger>
						);
					})}
				</TabsList>
			</div>
		</Tabs>
	);
}
