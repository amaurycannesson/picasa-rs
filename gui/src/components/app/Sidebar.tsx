import { Link } from '@tanstack/react-router';
import { Album, Image, Import, LucideUser, MapPin, Search, Settings } from 'lucide-react';

import {
  Sidebar as UiSidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from '@/components/ui/sidebar';

const mainMenuItems = [
  { icon: Search, label: 'Photos', path: '/search/gallery' },
  { icon: Album, label: 'Albums', path: '/' },
  { icon: LucideUser, label: 'People', path: '/people' },
  { icon: MapPin, label: 'Places', path: '/' },
];

const footerMenuItems = [
  { icon: Import, label: 'Import', path: '/' },
  { icon: Settings, label: 'Settings', path: '/' },
];

export function Sidebar() {
  return (
    <UiSidebar>
      <SidebarHeader>
        <SidebarHeader>
          <SidebarMenu>
            <SidebarMenuItem>
              <SidebarMenuButton asChild className="data-[slot=sidebar-menu-button]:!p-1.5">
                <a href="#">
                  <Image className="!size-5" />
                  <span className="text-base font-semibold">Picasa</span>
                </a>
              </SidebarMenuButton>
            </SidebarMenuItem>
          </SidebarMenu>
        </SidebarHeader>
      </SidebarHeader>
      <SidebarContent>
        <SidebarGroup>
          <SidebarMenu>
            {mainMenuItems.map((item) => (
              <SidebarMenuItem key={item.label}>
                <Link to={item.path}>
                  {({ isActive }) => (
                    <SidebarMenuButton asChild isActive={isActive}>
                      <span>
                        <item.icon />
                        <span>{item.label}</span>
                      </span>
                    </SidebarMenuButton>
                  )}
                </Link>
              </SidebarMenuItem>
            ))}
          </SidebarMenu>
        </SidebarGroup>
      </SidebarContent>
      <SidebarFooter>
        <SidebarMenu>
          {footerMenuItems.map((item) => (
            <SidebarMenuItem key={item.label}>
              <Link to={item.path}>
                {({ isActive }) => (
                  <SidebarMenuButton asChild isActive={isActive}>
                    <span>
                      <item.icon />
                      <span>{item.label}</span>
                    </span>
                  </SidebarMenuButton>
                )}
              </Link>
            </SidebarMenuItem>
          ))}
        </SidebarMenu>
      </SidebarFooter>
    </UiSidebar>
  );
}
