import { Box, Heading, Section as RadSection, Text } from "@radix-ui/themes";

type SectionProps = {
  children?: React.ReactNode;
  title: string;
};

export const Section = ({ children, title }: SectionProps) => {
  return (
    <RadSection py={"0"}>
      <Box py={"0"} asChild>
        <Heading
          size={"5"}
          as={"h2"}
          style={{
            display: "block",
            textTransform: "uppercase",
          }}
        >
          {title}
        </Heading>
      </Box>
      {children}
    </RadSection>
  );
};
