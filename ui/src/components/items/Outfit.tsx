import { Tech as TechType, Outfit as OutfitType } from "resources";
import { StatTable } from "../StatTable";
import { ItemCard } from "./ItemCard";
import { SimpleAction } from "../actions/SimpleAction";

type OutfitProps = { outfit: OutfitType; name: string };

export const Outfit = ({ outfit, name }: OutfitProps) => {
  return (
    <ItemCard item={outfit} name={name}>
      {{
        bottom: (
          <>
            {Object.entries(outfit.actions || {}).map(([name, action]) => (
              <SimpleAction name={name} action={action} key={name} />
            ))}
          </>
        ),
      }}
    </ItemCard>
  );
};
