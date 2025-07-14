import { zodResolver } from '@hookform/resolvers/zod';
import { createFileRoute, Outlet } from '@tanstack/react-router';
import { CheckIcon, ChevronsUpDownIcon } from 'lucide-react';
import * as React from 'react';
import { useForm } from 'react-hook-form';
import { z } from 'zod';

import { commands } from '@/bindings';
import { DatePicker } from '@/components/app/DatePicker';
import { ErrorMessage } from '@/components/app/ErrorMessage';
import { PersonCombobox } from '@/components/app/PersonCombobox';
import { Button } from '@/components/ui/button';
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from '@/components/ui/command';
import { Form, FormControl, FormField, FormItem } from '@/components/ui/form';
import { Input } from '@/components/ui/input';
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover';
import { cn, createSmartFilter } from '@/lib/utils';
import { photoSearchSchema } from '@/photoSearch';

const searchFormSchema = z.object({
  text: z.string().optional(),
  country_id: z.string().optional(),
  city_id: z.string().optional(),
  person_id: z.string().optional(),
  date_from: z.string().optional(),
  date_to: z.string().optional(),
});

type SearchFormValues = z.infer<typeof searchFormSchema>;

export const Route = createFileRoute('/search')({
  component: SearchPage,
  validateSearch: photoSearchSchema,
  loader: async () => {
    const result = await commands.getSearchOptions();

    if (result.status === 'error') throw new Error(result.error);

    return {
      breadcrumb: 'Photo search',
      searchOptions: result.data,
    };
  },
  errorComponent: ErrorMessage,
});

function SearchPage() {
  const { searchOptions } = Route.useLoaderData();
  const search = Route.useSearch();

  const navigate = Route.useNavigate();

  const form = useForm<SearchFormValues>({
    resolver: zodResolver(searchFormSchema),
    defaultValues: {
      text: search.text || '',
      country_id: search.country_id ? String(search.country_id) : '',
      city_id: search.city_id ? String(search.city_id) : '',
      person_id: search.person_id ? String(search.person_id) : '',
      date_from: search.date_from || '',
      date_to: search.date_to || '',
    },
  });

  const onSubmit = (values: SearchFormValues) => {
    navigate({
      to: '/search/gallery',
      search: {
        ...photoSearchSchema.parse(values),
      },
    });
  };

  return (
    <div>
      <Form {...form}>
        <form onSubmit={form.handleSubmit(onSubmit)} className="grid grid-cols-4 gap-2 pb-4">
          <FormField
            control={form.control}
            name="text"
            render={({ field }) => (
              <FormItem>
                <FormControl>
                  <Input placeholder="Search..." {...field} />
                </FormControl>
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="country_id"
            render={({ field }) => (
              <CountryCombobox field={field} countries={searchOptions.countries} />
            )}
          />
          <FormField
            control={form.control}
            name="city_id"
            render={({ field }) => <CityCombobox field={field} cities={searchOptions.cities} />}
          />
          <FormField
            control={form.control}
            name="person_id"
            render={({ field }) => (
              <PersonCombobox
                field={field}
                persons={searchOptions.persons}
                placeholder="Person..."
              />
            )}
          />
          <FormField
            control={form.control}
            name="date_from"
            render={({ field }) => (
              <FormItem>
                <FormControl>
                  <DatePicker
                    value={field.value}
                    onChange={field.onChange}
                    placeholder="From date..."
                  />
                </FormControl>
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="date_to"
            render={({ field }) => (
              <FormItem>
                <FormControl>
                  <DatePicker
                    value={field.value}
                    onChange={field.onChange}
                    placeholder="To date..."
                  />
                </FormControl>
              </FormItem>
            )}
          />
          <Button type="submit">Search</Button>
          <button
            type="button"
            onClick={() => form.reset()}
            className="flex items-center justify-start text-sm text-gray-500 underline hover:text-gray-700"
          >
            Reset
          </button>
        </form>
      </Form>
      <Outlet />
    </div>
  );
}

interface CountryComboboxProps {
  field: {
    value: string | undefined;
    onChange: (value: string) => void;
  };
  countries: Array<{ id: number; name: string | null }>;
}

function CountryCombobox({ field, countries }: CountryComboboxProps) {
  const [countryOpen, setCountryOpen] = React.useState(false);

  return (
    <FormItem>
      <Popover open={countryOpen} onOpenChange={setCountryOpen}>
        <FormControl>
          <PopoverTrigger asChild>
            <Button
              variant="outline"
              role="combobox"
              aria-expanded={countryOpen}
              className="w-full justify-between"
            >
              {field.value ? (
                countries.find((country) => String(country.id) === field.value)?.name ||
                `Country ${field.value}`
              ) : (
                <span className="text-muted-foreground">Country...</span>
              )}
              <ChevronsUpDownIcon className="ml-2 h-4 w-4 shrink-0 opacity-50" />
            </Button>
          </PopoverTrigger>
        </FormControl>
        <PopoverContent className="w-[var(--radix-popover-trigger-width)] p-0">
          <Command filter={createSmartFilter(countries, (country) => country.name || '')}>
            <CommandInput placeholder="Search country..." />
            <CommandList>
              <CommandEmpty>No country found.</CommandEmpty>
              <CommandGroup>
                {countries.map((country) => (
                  <CommandItem
                    key={country.id}
                    value={String(country.id)}
                    onSelect={(currentValue) => {
                      field.onChange(currentValue === field.value ? '' : currentValue);
                      setCountryOpen(false);
                    }}
                  >
                    <CheckIcon
                      className={cn(
                        'mr-2 h-4 w-4',
                        field.value === String(country.id) ? 'opacity-100' : 'opacity-0',
                      )}
                    />
                    {country.name || `Country ${country.id}`}
                  </CommandItem>
                ))}
              </CommandGroup>
            </CommandList>
          </Command>
        </PopoverContent>
      </Popover>
    </FormItem>
  );
}

interface CityComboboxProps {
  field: {
    value: string | undefined;
    onChange: (value: string) => void;
  };
  cities: Array<{ id: number; name: string }>;
}

function CityCombobox({ field, cities }: CityComboboxProps) {
  const [cityOpen, setCityOpen] = React.useState(false);

  return (
    <FormItem>
      <Popover open={cityOpen} onOpenChange={setCityOpen}>
        <FormControl>
          <PopoverTrigger asChild>
            <Button
              variant="outline"
              role="combobox"
              aria-expanded={cityOpen}
              className="w-full justify-between"
            >
              {field.value ? (
                cities.find((city) => String(city.id) === field.value)?.name ||
                `City ${field.value}`
              ) : (
                <span className="text-muted-foreground">City...</span>
              )}
              <ChevronsUpDownIcon className="ml-2 h-4 w-4 shrink-0 opacity-50" />
            </Button>
          </PopoverTrigger>
        </FormControl>
        <PopoverContent className="w-[var(--radix-popover-trigger-width)] p-0">
          <Command filter={createSmartFilter(cities, (city) => city.name)}>
            <CommandInput placeholder="Search city..." />
            <CommandList>
              <CommandEmpty>No city found.</CommandEmpty>
              <CommandGroup>
                {cities.map((city) => (
                  <CommandItem
                    key={city.id}
                    value={String(city.id)}
                    onSelect={(currentValue) => {
                      field.onChange(currentValue === field.value ? '' : currentValue);
                      setCityOpen(false);
                    }}
                  >
                    <CheckIcon
                      className={cn(
                        'mr-2 h-4 w-4',
                        field.value === String(city.id) ? 'opacity-100' : 'opacity-0',
                      )}
                    />
                    {city.name}
                  </CommandItem>
                ))}
              </CommandGroup>
            </CommandList>
          </Command>
        </PopoverContent>
      </Popover>
    </FormItem>
  );
}
