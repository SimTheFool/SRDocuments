import { MasonryGrid } from "@/components/MasonryGrid";
import { Space } from "@/components/Space";
import { TitleSection } from "@/components/TitleSection";
import { BaseAction } from "@/components/actions/BaseAction";
import { Sprite } from "@/components/items/Sprite";
import { Box } from "@radix-ui/themes";
import { characters } from "resources";
import { PdfContainer } from "../../PdfContainer";

type Props = {
  params: {
    name: string;
  };
};

export default function Home({ params: { name } }: Props) {
  const char = characters[name];

  return (
    <PdfContainer>
      <Box pt={"2"}>
        <MasonryGrid columns={3}>
          <Box>
            <TitleSection>Pouvoirs et compagnons</TitleSection>
            <Space />
          </Box>
          {Object.entries(char.sprites || {}).map(([name, sprite]) => {
            return <Sprite name={name} sprite={sprite} key={name} />;
          })}
          {Object.entries(char.actions || {}).map(([name, action]) => {
            return (
              <BaseAction
                name={name}
                action={action}
                type={action.type}
                key={name}
              />
            );
          })}
        </MasonryGrid>
      </Box>
    </PdfContainer>
  );
}
