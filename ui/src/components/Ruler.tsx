import { Box } from "@radix-ui/themes";

type RulerProps = {
  items: (string | number)[];
  placeholders?: (string | number)[];
};

export const Ruler = ({ items, placeholders }: RulerProps) => {
  return (
    <Box
      style={{
        borderBottom: "1px solid black",
        display: "inline-block",
      }}
    >
      {items.map((nb, i) => (
        <Box
          pr={i == items.length - 1 ? "0" : "2"}
          style={{
            display: "inline-block",
          }}
        >
          <Box
            style={{
              position: "relative",
            }}
          >
            <Box
              style={{
                visibility: "hidden",
              }}
            >
              {placeholders ? placeholders?.[i] : nb}
            </Box>
            <Box
              style={{
                position: "absolute",
                top: 0,
                right: 0,
              }}
            >
              {nb}
            </Box>
          </Box>

          <Box
            pb={"1"}
            style={{
              borderLeft: i == 0 ? "1px solid black" : "none",
              borderRight: i != 0 ? "1px solid black" : "none",
            }}
          />
        </Box>
      ))}
    </Box>
  );
};
