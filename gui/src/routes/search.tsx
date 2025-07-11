import { zodResolver } from '@hookform/resolvers/zod';
import { createFileRoute, Outlet } from '@tanstack/react-router';
import { useForm } from 'react-hook-form';
import { z } from 'zod';

import { commands } from '@/bindings';
import { ErrorMessage } from '@/components/app/ErrorMessage';
import { Button } from '@/components/ui/button';
import { Form, FormControl, FormField, FormItem } from '@/components/ui/form';
import { Input } from '@/components/ui/input';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { photoSearchSchema } from '@/photoSearch';

const searchFormSchema = z.object({
  text: z.string().optional(),
  country_id: z.string().optional(),
  city_id: z.string().optional(),
  person_id: z.string().optional(),
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

  const navigate = Route.useNavigate();

  const form = useForm<SearchFormValues>({
    resolver: zodResolver(searchFormSchema),
    defaultValues: {
      text: '',
      country_id: '',
      city_id: '',
      person_id: '',
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
        <form onSubmit={form.handleSubmit(onSubmit)} className="flex gap-2">
          <FormField
            control={form.control}
            name="text"
            render={({ field }) => (
              <FormItem className="flex-1">
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
              <FormItem className="flex-1">
                <Select onValueChange={field.onChange} value={field.value}>
                  <FormControl>
                    <SelectTrigger className="w-full">
                      <SelectValue placeholder="Country..." />
                    </SelectTrigger>
                  </FormControl>
                  <SelectContent>
                    {searchOptions.countries.map((country) => (
                      <SelectItem key={country.id} value={String(country.id)}>
                        {country.name || `Country ${country.id}`}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="city_id"
            render={({ field }) => (
              <FormItem className="flex-1">
                <Select onValueChange={field.onChange} value={field.value}>
                  <FormControl>
                    <SelectTrigger className="w-full">
                      <SelectValue placeholder="City..." />
                    </SelectTrigger>
                  </FormControl>
                  <SelectContent>
                    {searchOptions.cities.map((city) => (
                      <SelectItem key={city.id} value={String(city.id)}>
                        {city.name}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="person_id"
            render={({ field }) => (
              <FormItem className="flex-1">
                <Select onValueChange={field.onChange} value={field.value}>
                  <FormControl>
                    <SelectTrigger className="w-full">
                      <SelectValue placeholder="Person..." />
                    </SelectTrigger>
                  </FormControl>
                  <SelectContent>
                    {searchOptions.persons.map((person) => (
                      <SelectItem key={person.id} value={String(person.id)}>
                        {person.name}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </FormItem>
            )}
          />
          <Button type="submit">Search</Button>
        </form>
        <button
          type="button"
          onClick={() => form.reset()}
          className="pb-4 text-sm text-gray-500 underline hover:text-gray-700"
        >
          Reset
        </button>
      </Form>
      <Outlet />
    </div>
  );
}
