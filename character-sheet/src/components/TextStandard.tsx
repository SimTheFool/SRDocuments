import { Theme, Text } from "@radix-ui/themes";

type TextStandard = { children?: React.ReactNode };
export const TextStandard = ({ children }: TextStandard) => {
  return (
    <Theme>
      <Text size={"2"} as="span" style={{ display: "block" }}>
        {children}
      </Text>
    </Theme>
  );
};
