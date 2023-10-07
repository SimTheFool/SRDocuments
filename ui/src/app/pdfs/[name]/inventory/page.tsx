import { MasonryGrid } from "@/components/MasonryGrid";
import { Space } from "@/components/Space";
import { TitleSection } from "@/components/TitleSection";
import { SimpleAction } from "@/components/actions/SimpleAction";
import { Drone } from "@/components/items/Drone";
import { ItemCard } from "@/components/items/ItemCard";
import { Tech } from "@/components/items/Tech";
import { Weapon } from "@/components/items/Weapon";
import { Box } from "@radix-ui/themes";
import { characters } from "resources";
import { PdfContainer } from "../../PdfContainer";

type Props = {
  params: {
    name: string;
  };
};

export default async function Page({ params: { name } }: Props) {
  const char = characters[name];

  return (
    <PdfContainer>
      <MasonryGrid columns={3}>
        <Box>
          <TitleSection>Inventaire</TitleSection>
          <Space />
        </Box>
        {Object.entries(char.drones || {}).map(([name, drone]) => {
          return (
            <Box pb={"2"} pr={"2"} key={name}>
              <Drone item={drone} name={name} />
            </Box>
          );
        })}
        {Object.entries(char.weapons || {}).map(([name, weapon]) => {
          return (
            <Box pb={"2"} pr={"2"} key={name}>
              <Weapon weapon={weapon} name={name} />
            </Box>
          );
        })}
        {Object.entries(char.outfits || {}).map(([name, outfit]) => {
          return (
            <Box pb={"2"} pr={"2"} key={name}>
              <ItemCard item={outfit} name={name}>
                {{
                  bottom: (
                    <>
                      {Object.entries(outfit.actions || {}).map(
                        ([name, action]) => (
                          <SimpleAction
                            name={name}
                            action={action}
                            key={name}
                          />
                        )
                      )}
                    </>
                  ),
                }}
              </ItemCard>
            </Box>
          );
        })}
        {Object.entries(char.tech || {}).map(([name, tech]) => {
          return (
            <Box pb={"2"} pr={"2"} key={name}>
              <Tech tech={tech} name={name} />
            </Box>
          );
        })}
      </MasonryGrid>
    </PdfContainer>
  );
}
