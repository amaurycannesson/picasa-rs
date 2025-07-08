import { Await, createFileRoute } from '@tanstack/react-router';
import { Check } from 'lucide-react';
import { useState } from 'react';

import { commands, PendingFaceReview, Person, Result } from '@/bindings';
import { FaceCrop } from '@/components/app/FaceCrop';
import { Avatar, AvatarFallback } from '@/components/ui/avatar';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Card, CardFooter } from '@/components/ui/card';
import {
  Carousel,
  CarouselContent,
  CarouselItem,
  CarouselNext,
  CarouselPrevious,
} from '@/components/ui/carousel';
import { Input } from '@/components/ui/input';

export const Route = createFileRoute('/people')({
  component: RouteComponent,
  staticData: { breadcrumb: 'People' },
  loader: async () => {
    const people = await commands.listPersons();
    const pendingReviewsPromise = commands.getPendingManualReviews();

    return {
      people: people.status === 'ok' ? people.data : null,
      pendingReviewsPromise,
    };
  },
});

function RouteComponent() {
  const { pendingReviewsPromise, people } = Route.useLoaderData();

  return (
    <div>
      <Await promise={pendingReviewsPromise} fallback={<>Loading...</>}>
        {(res: Result<PendingFaceReview[], null>) =>
          res.status === 'ok' && res.data.length > 0 && <PendingReview faces={res.data} />
        }
      </Await>
      <People people={people} />
    </div>
  );
}

const People = ({ people }: { people: Person[] | null }) => {
  return (
    <div>
      <h4 className="scroll-m-20 text-xl font-semibold tracking-tight">People</h4>
      {(!people || people.length === 0) && <>No people found.</>}
      {people && people.map((person) => <_Person key={person.id} person={person} />)}
    </div>
  );
};

const _Person = ({ person }: { person: Person }) => {
  return (
    <div className="inline-block">
      <div className="flex flex-col items-center space-y-2 py-2">
        <Avatar className="size-16">
          <AvatarFallback className="text-2xl">
            {person.name.charAt(0).toUpperCase()}
          </AvatarFallback>
        </Avatar>
        <span className="text-center">{person.name}</span>
      </div>
    </div>
  );
};

const CardReview = ({
  face,
  onCreatePerson,
}: {
  face: PendingFaceReview;
  onCreatePerson: (personName: string) => void;
}) => {
  const [personName, setPersonName] = useState('');

  const handleCreatePerson = () => {
    if (personName) {
      onCreatePerson(personName);
    }
  };

  return (
    <Card className="relative w-1/3 px-2">
      <Carousel
        opts={{
          align: 'start',
        }}
        className="w-full"
      >
        <CarouselContent className="pl-2">
          {face.face_ids.map((id) => (
            <CarouselItem key={id} className="md:basis-1/2 lg:basis-1/3">
              <FaceCrop faceId={id} />
            </CarouselItem>
          ))}
        </CarouselContent>
        {face.face_ids.length > 3 && (
          <>
            <CarouselPrevious className="absolute top-1/2 left-2 -translate-y-1/2" />
            <CarouselNext className="absolute top-1/2 right-2 -translate-y-1/2" />
          </>
        )}
      </Carousel>

      <CardFooter>
        <Input
          placeholder="Enter person name"
          value={personName}
          onChange={(e) => setPersonName(e.target.value)}
        />
        <Button className="ml-2" onClick={handleCreatePerson} disabled={!personName}>
          <Check />
        </Button>
      </CardFooter>
    </Card>
  );
};

const PendingReview = ({ faces }: { faces: PendingFaceReview[] }) => {
  const handleCreatePerson = async (face: PendingFaceReview, personName: string) => {
    try {
      await commands.createPersonFromFaces(personName, face.face_ids);
      // Optionally refresh the data or show success message
    } catch (error) {
      console.error('Failed to create person:', error);
    }
  };

  return (
    <>
      <h4 className="scroll-m-20 text-xl font-semibold tracking-tight">
        Pending review <Badge>{faces.length}</Badge>
      </h4>
      <div className="pt-2">
        {faces.map((face) => (
          <CardReview
            key={face.cluster_id}
            face={face}
            onCreatePerson={(personName) => handleCreatePerson(face, personName)}
          />
        ))}
      </div>
    </>
  );
};
